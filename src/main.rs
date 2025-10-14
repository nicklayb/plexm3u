use clap::{Parser, Subcommand};
use log::LevelFilter;
use log::{debug, error, info};
use plex_client::{MediaContainer, PlexClient};

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

#[derive(Subcommand)]
enum Command {
    ListPlaylists,
    GetPlaylist { rating_key: String },
}

fn main() {
    let args = Cli::parse();
    configure_logger(&args);

    let plex_client = PlexClient::new(args.server, args.token);

    match args.command {
        Some(Command::ListPlaylists) => list_playlists(plex_client),
        Some(Command::GetPlaylist { rating_key }) => get_playlist(plex_client, rating_key),
        None => error!("No command provided"),
    }
}

fn get_playlist(plex_client: PlexClient, rating_key: String) {
    plex_client.get_playlist(rating_key);
}

fn list_playlists(plex_client: PlexClient) {
    let container = plex_client.list_playlists();
    println!("Found {} playlists", container.size);
    for playlist in container.playlists {
        println!("{}", playlist.to_string());
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
