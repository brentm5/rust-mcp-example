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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn formats_utc_datetime() {
        let dt = Utc.with_ymd_and_hms(2024, 6, 15, 14, 30, 0).unwrap();
        assert_eq!(format_time(dt), "Saturday June 15 2024 02:30 PM");
    }

    #[test]
    fn formats_midnight_as_12_am() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(format_time(dt), "Monday January 1 2024 12:00 AM");
    }

    #[test]
    fn formats_noon_as_12_pm() {
        let dt = Utc.with_ymd_and_hms(2024, 3, 20, 12, 0, 0).unwrap();
        assert_eq!(format_time(dt), "Wednesday March 20 2024 12:00 PM");
    }

    #[test]
    fn formats_single_digit_day_without_leading_zero() {
        let dt = Utc.with_ymd_and_hms(2024, 11, 5, 9, 5, 0).unwrap();
        assert_eq!(format_time(dt), "Tuesday November 5 2024 09:05 AM");
    }
}
