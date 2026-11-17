mod subcommands;

use clap::{Args, Parser, Subcommand};
use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pigeon", version = "0.1", about = "Send files locally", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Send(SendArgs),
    Receive,
    Peers,
    Doctor,
}

#[derive(Args)]
struct SendArgs {
    file: PathBuf,

    // flatten, so target is treated as a "Container of Arguments"
    #[command(flatten)]
    target: Target,

    // will replace default value with port from 'protocoll' crate
    #[arg(short, long, default_value_t = 52530)]
    port: u16,
}
// If peer name is not found, an ip adress can be provided with --ip
#[derive(Args, Clone)]
#[group(required = true, multiple = false)]
struct Target {
    peer: String,

    #[arg(long)]
    ip: Option<IpAddr>,
}

fn main() -> anyhow::Result<()> {
    let cli = CLI::parse();

    match &cli.command {
        Commands::Send(args) => subcommands::send::send(args),
        Commands::Receive => subcommands::receive::receive(),
        Commands::Peers => subcommands::peers::list(),
        Commands::Doctor => subcommands::doctor::run(),
    }
}
