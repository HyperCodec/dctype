use clap::Parser;
use std::path::PathBuf;
use std::fs;

mod discord;

#[derive(Parser)]
#[command(name="dctype", about="Infinitely types in a set of Discord channels")]
struct Cli {
    #[clap(short, long, help = "Channel list file")]
    channel_list: PathBuf,

    #[clap(short, long, help = "Discord Auth Token")]
    token: String,
}

#[tokio::main]
async fn main() {
    simple_logger::init().expect("Failed to initialize logger");

    let args = Cli::parse();

    log::info!("Initializing client");
    let client = discord::DiscordClient::new(args.token.clone());

    log::info!("Reading channel list from {}", args.channel_list.display());
    let channels_file = fs::read_to_string(args.channel_list).expect("Unable to read file");

    let channels: Vec<_> = channels_file.lines().map(|s| s.to_string()).collect();

    let mut handles = Vec::with_capacity(channels.len());

    log::info!("Starting typing in {} channels", channels.len());
    for c in channels {
        let c2 = client.clone();
        handles.push(tokio::spawn(async move {
            c2.infinitely_type(c).await;
        }));
    }

    for h in handles {
        h.await.expect("Failed to join async task");
    }
}