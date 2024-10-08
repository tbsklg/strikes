use clap::Parser;
use strikes::cli::{Cli, Command};
use strikes::clients::client::StrikeClient;
use strikes::clients::local_client::LocalClient;
use strikes::clients::remote_client::RemoteClient;
use strikes::configuration::{get_configuration, Settings};
use strikes::output::{print_as_table, print_strikes};
use strikes::tarnished::Tarnished;

#[tokio::main]
async fn main() {
    let args = &Cli::parse();
    let settings = &get_configuration(args);
    let client = create_client(settings);

    match &args.clone().command {
        Some(Command::Strike { name }) => match client.add_strike(name).await {
            Ok(strikes) => print_strikes(name, strikes),
            Err(err) => eprintln!("Failed to add strike: {}", err),
        },
        Some(Command::Ls) => match client.get_tarnished().await {
            Ok(tarnished) => print_as_table(Tarnished::sort_desc_by_strike(tarnished)),
            Err(err) => eprintln!("Failed to get strikes: {}", err),
        },
        Some(Command::Clear) => match client.clear_strikes().await {
            Ok(()) => println!("All strikes have been cleared!"),
            Err(err) => eprintln!("Faild to clear all strikes: {}", err),
        },
        Some(Command::CheckHealth) => match client.check_health().await {
            Ok(_) => println!("Everything is fine!"),
            Err(err) => eprintln!("Failed to check health: {}", err),
        },
        None => {
            eprintln!("No supported command was provided");
        }
    }
}

fn create_client(settings: &Settings) -> Box<dyn StrikeClient> {
    settings.remote.as_ref().map_or_else(
        || {
            Box::new(LocalClient {
                db_path: settings.local.as_ref().unwrap().db_path.clone(),
            }) as Box<dyn StrikeClient>
        },
        |remote| {
            Box::new(RemoteClient {
                api_key: remote.api_key.clone(),
                base_url: remote.base_url.clone(),
            }) as Box<dyn StrikeClient>
        },
    )
}
