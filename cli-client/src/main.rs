use clap::Parser;
use std::path::PathBuf;
use strike::client::get_example;
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
    let api_key = Config::parse(args.config_path.unwrap_or(path)).api_key;

    get_example().await;

    println!("args: {:?}", api_key);
}
