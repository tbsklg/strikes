use clap::{Parser, Subcommand};
use std::path::PathBuf;
use strikes::{
    configuration::get_configuration,
    local_client::{add_strike, get_tarnished},
    output::print_as_table,
};

#[derive(Subcommand, Clone, Debug)]
enum Command {
    #[command(about = "Adds a strike to the specified person.")]
    Strike { name: String },
    #[command(about = "Lists all the persons and the number of strikes they have")]
    Ls,
}

#[derive(Debug, Parser)]
#[command(
    name = "Strikes CLI",
    version = "0.0.2-alpha",
    about = "Manage strikes for people",
    long_about = "This is a command-line tool for blaming people."
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
            print_as_table(get_tarnished(&db_path));
        }
    }
}
