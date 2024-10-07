use clap::{Parser, Subcommand};

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    #[command(about = "Add a strike", alias = "s")]
    Strike {
        #[arg(help = "Name of the tarnished", value_parser = parse_username)]
        name: String,
    },
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
    long_about = "Simple CLI tool to track and assign strikes",
    arg_required_else_help = true
)]
pub struct Cli {
    #[arg(short, long, help = "Specify the path to the configuration file")]
    pub config_path: Option<std::path::PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

fn parse_username(s: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync + 'static>> {
    if s.is_empty() {
        return Err("Username cannot be empty".into());
    }

    if s.len() > 20 {
        return Err("Username cannot be longer than 20 characters".into());
    }

    Ok(s.to_lowercase())
}
