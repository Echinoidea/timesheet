use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use index::{load_timesheet_index, serialize_index_hashmap};
use std::{collections::HashMap, path::Path};
use timesheet::Timesheet;

pub mod index;
pub mod project;
pub mod query;
pub mod timesheet;

/// Command usage should look like this
/// timesheet clock midas (for clock in if closed)
/// timesheet clock midas (for clock out if open)
/// timesheet mk-project i-begin
/// timesheet rm-project i-begin
/// timesheet query range midas <start yyyy-mm-dd> (optional, default NOW) <end yyyy-mm-dd> (optional) --as-csv (returns total hours worked)
/// timesheet query two-weeks midas

/// Clap subcommands
#[derive(Subcommand)]
enum Subcommands {
    Clock {
        project_name: String,
        message: Option<String>,
    },
    Project {
        command: String,
        project_name: String,
    },
    Query {
        timeframe: String,
        project_name: String,
    },
}

#[derive(Parser)]
#[command(name = "timesheet")]
#[command(version = "0.0.1")]
#[command(about = "Clock in and out of projects.")]
struct Args {
    #[command(subcommand)]
    command: Subcommands,
}

fn main() {
    let args = Args::parse();
    let command = &args.command;

    let mut index_map: HashMap<String, String> =
        load_timesheet_index(Path::new("/home/gabriel/timesheets/index.json")).unwrap();

    match command {
        Subcommands::Clock {
            project_name,
            message,
        } => {
            let mut timesheet: Timesheet = Timesheet::load_timesheet(Path::new(
                index_map
                    .get(project_name)
                    .expect("Timesheet file not found!"),
            ));

            let message = match message {
                Some(m) => m,
                None => &"".to_string(),
            };

            if timesheet.is_clocked_in() {
                timesheet.clock_out(&message);
            } else {
                timesheet.clock_in(&message);
            }

            timesheet.serialize(Path::new(
                index_map
                    .get(project_name)
                    .expect("Failed during serialization"),
            ));
        }

        Subcommands::Project {
            command,
            project_name,
        } => {
            match command.to_lowercase().as_str() {
                "new" => {
                    index_map.insert(
                        project_name.to_string(),
                        format!("/home/gabriel/timesheets/{}.json", project_name).to_string(),
                    );

                    serialize_index_hashmap(
                        index_map,
                        Path::new("/home/gabriel/timesheets/index.json"),
                    );
                }
                "remove" => {
                    index_map.remove(project_name);
                    serialize_index_hashmap(
                        index_map,
                        Path::new("/home/gabriel/timesheets/index.json"),
                    );
                }
                _ => {}
            };
        }

        Subcommands::Query {
            timeframe,
            project_name,
        } => todo!(),
    }
}
