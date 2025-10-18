use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

use clap::{Args, Parser, Subcommand};
use log::LevelFilter;
use log::error;
use plex_client::PlexClient;

use crate::m3u::Item;
use crate::m3u::M3U;
use crate::m3u::WithMetadata;
use crate::plex_client::playlist::PlaylistFilter;
use crate::plex_client::track::WithMedia;

mod m3u;
mod plex_client;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Args)]
struct DumpPlaylistArguments {
    #[arg(short, long)]
    server: String,
    #[arg(short, long)]
    token: Option<String>,
    rating_key: String,
    #[arg(long)]
    rewrite_from: Option<String>,
    #[arg(long)]
    rewrite_to: Option<String>,
    #[arg(long, short)]
    file: Option<String>,
    #[arg(long)]
    stdout: bool,
}

#[derive(Debug, Args)]
struct GetPlaylistArguments {
    #[arg(short, long)]
    server: String,
    #[arg(short, long)]
    token: Option<String>,
    rating_key: String,
}

#[derive(Debug, Args)]
struct PlaylistsFilterArguments {
    #[arg(short, long)]
    server: String,
    #[arg(short, long)]
    token: Option<String>,
    #[arg(long)]
    only: Option<String>,
}

#[derive(Debug, Args)]
struct VerifyM3uArguments {
    #[arg(long, short)]
    file: String,
    #[arg(long, short)]
    path: Option<String>,
    #[arg(long)]
    fix: bool,
    #[arg(short, long)]
    token: Option<String>,
    #[arg(short, long)]
    server: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    ListPlaylists(PlaylistsFilterArguments),
    GetPlaylist(GetPlaylistArguments),
    DumpPlaylist(DumpPlaylistArguments),
    VerifyM3u(VerifyM3uArguments),
}

fn main() {
    let args = Cli::parse();
    configure_logger(&args);

    match args.command {
        Some(Command::ListPlaylists(list_playlists_arguments)) => {
            let plex_client = PlexClient::new(
                list_playlists_arguments.server.clone(),
                list_playlists_arguments.token.clone(),
            );
            list_playlists(plex_client, list_playlists_arguments)
        }
        Some(Command::GetPlaylist(get_playlist_arguments)) => {
            let plex_client = PlexClient::new(
                get_playlist_arguments.server.clone(),
                get_playlist_arguments.token.clone(),
            );
            get_playlist(plex_client, get_playlist_arguments)
        }
        Some(Command::DumpPlaylist(dump_playlist_arguments)) => {
            let plex_client = PlexClient::new(
                dump_playlist_arguments.server.clone(),
                dump_playlist_arguments.token.clone(),
            );
            dump_playlist(plex_client, dump_playlist_arguments)
        }
        Some(Command::VerifyM3u(verify_m3u_arguments)) => verify_m3u(verify_m3u_arguments),
        None => error!("No command provided"),
    }
}

fn verify_m3u(arguments: VerifyM3uArguments) {
    let root_path = arguments
        .path
        .as_deref()
        .map(Path::new)
        .unwrap_or_else(|| Path::new(&arguments.file).parent().unwrap());

    let should_fix = match (arguments.server.clone(), arguments.fix) {
        (Some(_), true) => true,
        (None, true) => {
            panic!("Must provide `--server` (and maybe `--token`) to be able to fix missing tracks")
        }
        _ => false,
    };

    let read_m3u = m3u::read(&arguments.file);
    let mut missing_tracks = vec![];
    let mut total_count = 0;
    match read_m3u {
        Ok(M3U { tracks, .. }) => {
            for track in tracks.iter() {
                total_count += 1;
                if !track.exists_at(root_path) {
                    missing_tracks.push(track.clone());
                }
            }
        }
        Err(error) => panic!("Could not read {}: {}", arguments.file, error),
    }

    if missing_tracks.is_empty() {
        println!("All tracks ({}) exists", total_count)
    } else {
        let missing_track_count = missing_tracks.len();
        for track in missing_tracks {
            println!("- {}", track.path);
            if should_fix {
                println!("\tDownloading...");
                let plex_client =
                    PlexClient::new(arguments.server.clone().unwrap(), arguments.token.clone());
                if !download_part(plex_client, track.clone(), root_path) {
                    println!("\tCould not download {}", track.path.clone());
                }
            }
        }

        println!(
            "\nMissing tracks at {:?}: {} / {}",
            root_path, missing_track_count, total_count,
        );
    }
}

fn download_part(plex_client: PlexClient, track: Item, root_path: &Path) -> bool {
    match track.track_key() {
        Some(key) => {
            let mut response = plex_client.get_part(key);
            let full_path = track.full_path(root_path);
            fs::create_dir_all(full_path.parent().unwrap()).expect("Folder could not be created");
            let mut out = File::create(full_path.clone()).expect("File could not be created");
            match io::copy(&mut response, &mut out) {
                Ok(_) => {
                    println!("\tCreated {:?}", full_path);
                    true
                }
                Err(error) => {
                    eprintln!("\tError occurred while copying {}", error);
                    false
                }
            }
        }
        None => false,
    }
}

fn dump_playlist(plex_client: PlexClient, arguments: DumpPlaylistArguments) {
    if let None = arguments.file
        && !arguments.stdout
    {
        panic!("Requires at least `--file [FILE]` or `--stdout`")
    }
    let container = plex_client.get_playlist(arguments.rating_key.clone());
    let tracks =
        container.track_files(arguments.rewrite_from.clone(), arguments.rewrite_to.clone());
    if let Some(file) = arguments.file {
        let destination_folder = Path::new(&file);

        let destination_file = if destination_folder.is_dir() {
            let destination_file = format!("{}.m3u", container.title);
            Path::new(&destination_folder).join(destination_file)
        } else {
            destination_folder.to_path_buf()
        };
        println!("Writing {:?}", destination_file);
        let mut metadata = container.metadata();
        if let Some(rewrite_from) = arguments.rewrite_from {
            metadata.push(m3u::Metadata::RewriteFrom(rewrite_from.clone()))
        }
        if let Some(rewrite_to) = arguments.rewrite_to {
            metadata.push(m3u::Metadata::RewriteTo(rewrite_to.clone()))
        }

        let m3u = M3U::new(tracks.clone(), metadata);
        if let Err(error) = m3u::write(destination_file.clone(), m3u) {
            panic!("Error writing {:?}: {}", destination_file, error);
        }
    }
    if arguments.stdout {
        for track in tracks.clone() {
            println!("{:?}", track);
        }
    }
}

fn get_playlist(plex_client: PlexClient, arguments: GetPlaylistArguments) {
    let container = plex_client.get_playlist(arguments.rating_key);
    let track_count = container.tracks.len();
    let video_count = container.videos.len();

    if track_count > 0 {
        println!("{} tracks", track_count);
        for track in container.tracks.iter() {
            println!("{}", track.full_title())
        }
    }

    if video_count > 0 {
        println!("{} videos", video_count);
        for video in container.videos.iter() {
            println!("{}", video.full_title())
        }
    }
}

fn list_playlists(plex_client: PlexClient, playlists_filter_arguments: PlaylistsFilterArguments) {
    let container = plex_client.list_playlists();
    println!("Found {} playlists", container.size);
    for playlist in container.playlists {
        if playlist.matches(&to_playlist_filter(&playlists_filter_arguments)) {
            println!("{}", playlist.to_string());
        }
    }
}

fn to_playlist_filter(playlists_filter_arguments: &PlaylistsFilterArguments) -> PlaylistFilter {
    PlaylistFilter {
        only_playlist_type: playlists_filter_arguments.only.clone(),
    }
}

fn configure_logger(args: &Cli) {
    colog::init();

    let level = if args.verbose {
        LevelFilter::Trace
    } else {
        LevelFilter::Error
    };

    log::set_max_level(level);
}
