use chrono::{Local, Utc};
use clap::Args;

#[derive(Args, Debug)]
pub struct TimeArgs {
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

pub fn run(args: &TimeArgs) {
    println!("{}", format_time(Local::now()));
    if args.utc {
        println!("UTC: {}", format_time(Utc::now()));
    }
}
