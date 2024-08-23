use clap::Parser;
use strikes::cli::{Cli, Command};
use strikes::clients::client::StrikeClient;
use strikes::clients::local_client::LocalClient;
use strikes::clients::remote_client::RemoteClient;
use strikes::configuration::{get_configuration, Settings};
use strikes::output::print_as_table;

#[tokio::main]
async fn main() {
    let args = &Cli::parse();
    let settings = &get_configuration(args);
    let client = create_client(settings);

    match &args.clone().command.unwrap() {
        Command::Strike { name } => {
            client.add_strike(name);
            println!("{} has been tarnished!", name);
        }
        Command::Ls => {
            let tarnished = client.get_tarnished();

            if tarnished.is_empty() {
                println!("No one has been tarnished yet!");
                return;
            }

            print_as_table(tarnished);
        }
        Command::Clear => {
            client.clear_strikes();
            println!("All strikes have been cleared!");
        }
        Command::CheckHealth => match client.check_health() {
            Ok(_) => println!("Everything is fine!"),
            Err(_) => println!("Client is not healthy!"),
        },
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
