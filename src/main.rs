use clap::{Parser, Subcommand};
use index::{load_timesheet_index, serialize_index_hashmap};
use std::{collections::HashMap, path::Path};
use timesheet::Timesheet;

pub mod index;
pub mod query;
pub mod timesheet;

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
        project_name: String,
        timeframe: String,
    },
}

#[derive(Parser)]
#[command(name = "timesheet")]
#[command(version = "0.1.0")]
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
            project_name,
            timeframe,
        } => {
            let timesheet_path = Path::new(
                index_map
                    .get(project_name)
                    .expect("Timesheet file not found!"),
            );

            query::query_time_range(timesheet_path, timeframe, project_name);
        }
    }
}
