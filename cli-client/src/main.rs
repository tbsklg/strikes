use clap::Parser;
use std::path::PathBuf;
use strike::client::check_health;
use strike::config::Config;

#[derive(Debug, Parser)]
struct Cli {
    name: String,
    config_path: Option<std::path::PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let home = std::env::var("HOME").unwrap();
    let path = PathBuf::from(home).join(".strike");
    let config = Config::parse(args.config_path.unwrap_or(path));

    check_health(config.base_url, config.api_key).await;
}
