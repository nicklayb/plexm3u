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
    server: String,
    #[arg(short, long)]
    token: Option<String>,
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Args)]
struct DumpPlaylistArguments {
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
    rating_key: String,
}

#[derive(Debug, Args)]
struct PlaylistsFilterArguments {
    #[arg(long)]
    only: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    ListPlaylists(PlaylistsFilterArguments),
    GetPlaylist(GetPlaylistArguments),
    DumpPlaylist(DumpPlaylistArguments),
}

fn main() {
    let args = Cli::parse();
    configure_logger(&args);

    let plex_client = PlexClient::new(args.server, args.token);

    match args.command {
        Some(Command::ListPlaylists(list_playlists_arguments)) => {
            list_playlists(plex_client, list_playlists_arguments)
        }
        Some(Command::GetPlaylist(get_playlist_argments)) => {
            get_playlist(plex_client, get_playlist_argments)
        }
        Some(Command::DumpPlaylist(dump_playlist_arguments)) => {
            dump_playlist(plex_client, dump_playlist_arguments)
        }
        None => error!("No command provided"),
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
            println!("{}", track);
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
