use clap::{Parser, Subcommand};

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    #[command(about = "Add a strike", alias = "s")]
    Strike { name: String },
    #[command(about = "List all strikes")]
    Ls,
    #[command(about = "Clear strikes", alias = "c")]
    Clear,
}

#[derive(Clone, Debug, Parser)]
#[command(
    name = "Strikes CLI",
    version = "0.1.0",
    about = "Track and assign strikes",
    long_about = "Simple CLI tool to track and assign strikes"
)]
pub struct Cli {
    #[arg(
        short,
        long,
        help = "Specify the path to the configuration file where the strikes are stored"
    )]
    pub config_path: Option<std::path::PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,
}
