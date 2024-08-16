use clap::Parser;
use std::path::PathBuf;
use strikes::{
    configuration::get_configuration, local_client::add_strike, remote_client::check_health,
};

#[derive(Debug, Parser)]
struct Cli {
    name: String,
    config_path: Option<std::path::PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let home = &std::env::var("HOME").unwrap();
    let config_path = PathBuf::from(home).join(".strikes/configuration.yaml");
    let config = get_configuration(args.config_path.unwrap_or(config_path))
        .expect("Faild to read configuration.");

    check_health(config.base_url, config.api_key).await;
    let db_path = PathBuf::from(home).join(".strikes/db.json");
    add_strike(&args.name, &db_path);
}
