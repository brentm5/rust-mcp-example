use clap::{Parser, Subcommand};

mod commands;
mod mcp_tools;
mod notes;

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
    /// Print system information
    SysInfo(commands::sys_info::SysInfoArgs),
    /// Start the MCP stdio server
    Mcp,
    /// Manage notes
    Notes(commands::notes::NotesArgs),
}

fn main() {
    let args = Cli::parse();

    if args.debug {
        println!("Debug mode is on");
    }

    match args.command {
        Commands::Time(time_args) => commands::time::run(&time_args),
        Commands::SysInfo(sys_info_args) => commands::sys_info::run(&sys_info_args),
        Commands::Mcp => commands::mcp::run(),
        Commands::Notes(notes_args) => commands::notes::run(&notes_args),
    }
}
