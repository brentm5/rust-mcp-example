use chrono::{Local, Utc};
use clap::{Args, Parser, Subcommand};

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
    Time(TimeArgs),
}

#[derive(Args, Debug)]
struct TimeArgs {
    /// Also print UTC time
    #[arg(long)]
    utc: bool,
}

fn format_time<Tz: chrono::TimeZone>(dt: chrono::DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    dt.format("%A %B %-d %Y %I:%M %p").to_string()
}

fn main() {
    let args = Cli::parse();

    if args.debug {
        println!("Debug mode is on");
    }

    match args.command {
        Commands::Time(time_args) => {
            println!("{}", format_time(Local::now()));
            if time_args.utc {
                println!("UTC: {}", format_time(Utc::now()));
            }
        }
    }
}
