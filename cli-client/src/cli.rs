use clap::{Parser, Subcommand};

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    #[command(about = "Add a strike", alias = "s")]
    Strike { name: String },
    #[command(about = "List all strikes")]
    Ls,
    #[command(about = "Clear strikes", alias = "c")]
    Clear,
    #[command(about = "Check health of the client", alias = "h")]
    CheckHealth,
}

#[derive(Clone, Debug, Parser)]
#[command(
    name = "Strikes CLI",
    version = env!("CARGO_PKG_VERSION"),
    about = "Track and assign strikes",
    long_about = "Simple CLI tool to track and assign strikes"
)]
pub struct Cli {
    #[arg(short, long, help = "Specify the path to the configuration file")]
    pub config_path: Option<std::path::PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,
}
