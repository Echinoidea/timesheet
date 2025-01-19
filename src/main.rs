use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Write, path::Path};

/// Command usage should look like this
/// timesheet clock midas (for clock in if closed)
/// timesheet clock midas (for clock out if open)
/// timesheet mk-project i-begin
/// timesheet rm-project i-begin
/// timesheet query range midas <start yyyy-mm-dd> (optional, default NOW) <end yyyy-mm-dd> (optional) --as-csv (returns total hours worked)
/// timesheet query two-weeks midas

/// Project name is self explanatory.
/// Project path is the path to the Timesheet JSON file.
#[derive(Serialize, Deserialize, Debug)]
struct ProjectIndex {
    name: String,
    path: String,
}

/// This stores a list of projects. This is serialized to JSON.
#[derive(Serialize, Deserialize, Debug)]
struct IndexFile {
    project_indexes: Vec<ProjectIndex>,
}

/// An entry in a project Timesheet.
#[derive(Serialize, Deserialize, Debug)]
struct TimesheetEntry {
    time_in: String,
    time_out: String,
    message: String,
}

/// A project Timesheet.
#[derive(Serialize, Deserialize, Debug)]
struct Timesheet {
    entries: Vec<TimesheetEntry>,
}

/// Clap subcommands
#[derive(Subcommand)]
enum Subcommands {
    Clock {
        project_name: String,
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

    let index_map: HashMap<String, String> =
        load_timesheet_index(Path::new("/home/gabriel/timesheets/index.json")).unwrap();

    match command {
        Subcommands::Clock { project_name } => {
            let mut timesheet: Timesheet =
                Timesheet::load_timesheet(Path::new(index_map.get(project_name).expect("Ahhhh")));

            if timesheet.is_clocked_in() {
                timesheet.clock_out();
            } else {
                timesheet.clock_in();
            }

            timesheet.serialize(Path::new(index_map.get("midas").expect("Ahhhh")));
        }

        Subcommands::Project {
            command,
            project_name,
        } => todo!(),

        Subcommands::Query {
            timeframe,
            project_name,
        } => todo!(),
    }
}

/// Attempt to load the project index JSON file. If it doesn't exist, initialize a new one.
fn load_timesheet_index(
    path: &Path,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut project_map: HashMap<String, String> = HashMap::new();

    // If index path file does not exist, create it.
    if !path
        .try_exists()
        .expect("Index path is indeterminate. You may not have read permission.")
    {
        eprintln!(
            "Project index path not found. Creating index file at {}",
            path.display()
        );

        // Initialize JSON structure with no contents
        let initial_index = IndexFile {
            project_indexes: Vec::new(),
        };
        let initial_json = serde_json::to_string_pretty(&initial_index)
            .expect("Failed to serialize initial index structure.");

        let mut file =
            File::create(path).expect("Creating project index path failed. Path is invalid.");

        // Write the empty formatted JSON
        file.write_all(initial_json.as_bytes())
            .expect("Failed to write initial JSON to file.");
    }

    // Then read the index file, it will be empty if it was not found in the previous section
    let data = std::fs::read_to_string(path).expect("Could not load index path.");

    let index: IndexFile = serde_json::from_str(&data).expect("Could not read index file JSON.");

    for project in index.project_indexes {
        project_map.insert(project.name, project.path);
    }

    Ok(project_map)
}

impl Timesheet {
    /// Load a project/timesheet JSON file and load it into a vector of TimesheetEntry
    fn load_timesheet(path: &Path) -> Timesheet {
        // If timesheet path file does not exist, create it.
        if !path
            .try_exists()
            .expect("Index path is indeterminate. You may not have read permission.")
        {
            eprintln!(
                "Project index path not found. Creating index file at {}",
                path.display()
            );

            // Initialize JSON structure with no contents
            let initial_timesheet = Timesheet {
                entries: Vec::new(),
            };

            let initial_json = serde_json::to_string_pretty(&initial_timesheet)
                .expect("Failed to serialize initial index structure.");

            let mut file =
                File::create(path).expect("Creating project index path failed. Path is invalid.");

            // Write the empty formatted JSON
            file.write_all(initial_json.as_bytes())
                .expect("Failed to write initial JSON to file.");
        }

        // Load the data, it will be empty if it was just created above
        let data = std::fs::read_to_string(&path).expect("Could not load timesheet path.");

        let timesheet: Timesheet =
            serde_json::from_str(&data).expect("Could not read timesheet file JSON.");

        let mut entries: Vec<TimesheetEntry> = vec![];

        for entry in timesheet.entries {
            entries.push(entry);
        }

        Timesheet { entries }
    }

    /// Write timesheet data to JSON
    fn serialize(self: &Self, path: &Path) {
        let data = serde_json::to_string_pretty(&self)
            .expect("Failed to serialize initial index structure.");

        let mut file = File::create(path).expect("Error creating file buffer to write timesheet");

        file.write_all(data.as_bytes())
            .expect("Failed to write initial JSON to file.");
    }

    //fn get_last_entry(self: &Self) -> Option<&TimesheetEntry> {
    //    match self.entries.last() {
    //        Some(entry) => return Some(&entry),
    //        None => None,
    //    }
    //}
    //
    //fn get_last_entry_mut(self: &mut Self) -> Option<&mut TimesheetEntry> {
    //    match self.entries.last_mut() {
    //        Some(entry) => return Some(entry),
    //        None => None,
    //    }
    //}
    //
    //fn is_empty(self: &mut Self) -> bool {
    //    self.entries.is_empty()
    //}

    fn is_clocked_in(self: &mut Self) -> bool {
        match self.entries.last() {
            Some(last_entry) => last_entry.time_out.is_empty(),
            None => false,
        }
    }

    fn is_clocked_out(self: &mut Self) -> bool {
        !self.is_clocked_in()
    }

    fn clock_in(self: &mut Self) {
        // Check if already clocked in
        if self.is_clocked_in() {
            println!("Already clocked in.");
            return;
        }

        let now = chrono::Local::now().to_rfc3339();

        match self.entries.last_mut() {
            Some(last_entry) => {
                if last_entry.time_out.is_empty() {
                    println!("Clock-in failed: Last entry is still open.");
                } else {
                    // If the last entry is clocked out, create a new entry and clock in
                    self.entries.push(TimesheetEntry {
                        time_in: now.clone(),
                        time_out: String::new(),
                        message: String::new(),
                    });
                    println!("Clocked into a new entry.");
                }
            }
            None => {
                // No entries yet, create a new one and clock in
                self.entries.push(TimesheetEntry {
                    time_in: now.clone(),
                    time_out: String::new(),
                    message: String::new(),
                });
                println!("Clocked in successfully.");
            }
        }
    }

    fn clock_out(self: &mut Self) {
        if self.is_clocked_out() {
            println!("Not clocked in yet")
        }

        let now = chrono::Local::now().to_rfc3339();

        // If there's no last entry or the last entry has a clock-out, create a new entry
        match self.entries.last_mut() {
            Some(last_entry) => {
                if last_entry.time_in.is_empty() {
                    println!("Clock-out failed: Last entry isn't clocked in.");
                } else {
                    last_entry.time_out = now.to_string();
                }
            }
            _ => (),
        }
    }
}
