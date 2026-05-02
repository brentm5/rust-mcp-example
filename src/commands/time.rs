use chrono::{Local, Utc};
use clap::Args;

#[derive(Args, Debug)]
pub struct TimeArgs {
    /// Print UTC time instead of local time
    #[arg(long)]
    utc: bool,
}

pub fn format_time<Tz: chrono::TimeZone>(dt: chrono::DateTime<Tz>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    dt.format("%A %B %-d %Y %I:%M %p").to_string()
}

pub fn run(args: &TimeArgs) {
    if args.utc {
        println!("{}", format_time(Utc::now()));
    } else {
        println!("{}", format_time(Local::now()));
    }
}
