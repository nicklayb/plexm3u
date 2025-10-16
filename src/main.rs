use std::path::Path;

use clap::{Args, Parser, Subcommand};
use log::LevelFilter;
use log::error;
use plex_client::PlexClient;

use crate::plex_client::playlist::PlaylistFilter;

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

    let read_tracks = m3u::read(&arguments.file);
    let mut missing_tracks = vec![];
    let mut total_count = 0;
    let mut missing_count = 0;
    match read_tracks {
        Ok(tracks) => {
            for track in tracks.iter() {
                total_count += 1;
                if !track.exists_at(root_path) {
                    missing_count += 1;
                    missing_tracks.push(track.clone());
                }
            }
        }
        Err(error) => panic!("Could not read {}: {}", arguments.file, error),
    }

    if missing_tracks.is_empty() {
        println!("All tracks ({}) exists", total_count)
    } else {
        println!(
            "The following tracks ({} out of {}) could not be found at {:?}",
            missing_count, total_count, root_path
        );
        for track in missing_tracks {
            println!("\t{}", track.path)
        }
    }
}

fn dump_playlist(plex_client: PlexClient, arguments: DumpPlaylistArguments) {
    if let None = arguments.file
        && !arguments.stdout
    {
        panic!("Requires at least `--file [FILE]` or `--stdout`")
    }
    let container = plex_client.get_playlist(arguments.rating_key.clone());
    let tracks = container.track_files(arguments.rewrite_from, arguments.rewrite_to);
    if let Some(file) = arguments.file {
        println!("Writing {}", file);
        if let Err(error) = m3u::write(file.clone(), tracks.clone()) {
            panic!("Error writing {}: {}", file, error);
        }
    }
    if arguments.stdout {
        for track in &tracks {
            println!("{:?}", track);
        }
    }
}

fn get_playlist(plex_client: PlexClient, arguments: GetPlaylistArguments) {
    let container = plex_client.get_playlist(arguments.rating_key);
    println!("{:?}", container);
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
