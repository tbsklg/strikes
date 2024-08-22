use clap::{Parser, Subcommand};
use std::path::PathBuf;
use strikes::{
    configuration::get_configuration,
    local_client::{add_strike, clear_strikes, get_tarnished},
    output::print_as_table,
};

#[derive(Subcommand, Clone, Debug)]
enum Command {
    #[command(about = "Add a strike", alias = "s")]
    Strike { name: String },
    #[command(about = "List all strikes")]
    Ls,
    #[command(about = "Clear strikes", alias = "c")]
    Clear,
}

#[derive(Debug, Parser)]
#[command(
    name = "Strikes CLI",
    version = "0.1.0",
    about = "Track and assign strikes",
    long_about = "Simple CLI tool to track and assign strikes"
)]
struct Cli {
    #[arg(
        short,
        long,
        help = "Specify the path to the configuration file where the strikes are stored"
    )]
    config_path: Option<std::path::PathBuf>,

    #[arg(
        short,
        long,
        help = "Specify the path to the database json file (i.e. db.json)"
    )]
    db_path: Option<std::path::PathBuf>,
    #[command(subcommand)]
    command: Option<Command>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let home = &std::env::var("HOME").unwrap();
    let config_path = PathBuf::from(home).join(".strikes/configuration.yaml");
    let config = get_configuration(args.config_path.unwrap_or(config_path)).unwrap_or_default();

    // check_health(config.base_url, config.api_key).await;
    let db_path = args.db_path.unwrap_or(config.local.map_or_else(
        || PathBuf::from(home).join(".strikes/db.json"),
        |local| local.db_path,
    ));

    match args.command.unwrap() {
        Command::Strike { name } => {
            add_strike(&name, &db_path);
            println!("{} has been tarnished!", name);
        }
        Command::Ls => {
            let tarnished = get_tarnished(&db_path);

            if tarnished.is_empty() {
                println!("No one has been tarnished yet!");
                return;
            }

            print_as_table(tarnished);
        }
        Command::Clear => {
            clear_strikes(&db_path);
            println!("All strikes have been cleared!");
        }
    }
}
