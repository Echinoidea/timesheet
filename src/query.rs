use crate::timesheet::Timesheet;
use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use std::path::Path;

/// Parse a date string in `yyyy-mm-dd` format.
fn parse_date(date: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()
}

/// Calculate the total hours worked within a time range.
pub fn calculate_hours_within_range(
    timesheet: &Timesheet,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> f64 {
    timesheet
        .entries
        .iter()
        .filter_map(|entry| {
            if let (Ok(start_time), Ok(end_time)) = (
                NaiveDateTime::parse_from_str(&entry.time_in, "%Y-%m-%dT%H:%M:%S%.f%:z"),
                NaiveDateTime::parse_from_str(&entry.time_out, "%Y-%m-%dT%H:%M:%S%.f%:z"),
            ) {
                // Check if the entry falls within the range
                if end_time >= start && start_time <= end {
                    let effective_start = if start_time < start {
                        start
                    } else {
                        start_time
                    };
                    let effective_end = if end_time > end { end } else { end_time };
                    Some((effective_end - effective_start).num_seconds() as f64 / 3600.0)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum()
}

/// Query a time range based on user input.
pub fn query_time_range(timesheet_path: &Path, timeframe: &str, project_name: &str) {
    let timesheet = Timesheet::load_timesheet(timesheet_path);

    // Calculate date range based on the timeframe
    let now = Local::now().naive_local();
    let (start, end) = match timeframe {
        "two-weeks" => (now - Duration::weeks(2), now),
        _ => {
            let parts: Vec<&str> = timeframe.split_whitespace().collect();
            if parts.len() == 2 {
                if let (Some(start_date), Some(end_date)) =
                    (parse_date(parts[0]), parse_date(parts[1]))
                {
                    (
                        start_date.and_hms_opt(0, 0, 0).unwrap(),
                        end_date.and_hms_opt(23, 59, 59).unwrap(),
                    )
                } else {
                    eprintln!("Invalid date range format. Use yyyy-mm-dd.");
                    return;
                }
            } else {
                eprintln!("Unsupported timeframe format.");
                return;
            }
        }
    };

    // Calculate total hours worked
    let total_hours = calculate_hours_within_range(&timesheet, start, end);
    println!(
        "Total hours worked on project '{}': {:.2} hours",
        project_name, total_hours
    );
}
