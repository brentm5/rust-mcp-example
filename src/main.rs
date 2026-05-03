use clap::{Parser, Subcommand};

mod commands;
mod mcp_tools;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Enable debug output
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Print the current time
    Time(commands::time::TimeArgs),
    /// Start the MCP stdio server
    Mcp,
}

fn main() {
    let args = Cli::parse();

    if args.debug {
        println!("Debug mode is on");
    }

    match args.command {
        Commands::Time(time_args) => commands::time::run(&time_args),
        Commands::Mcp => commands::mcp::run(),
    }
}
