use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
    str::FromStr,
};

/// Command usage should look like this
/// timesheet clock midas (for clock in if closed)
/// timesheet clock midas (for clock out if open)
/// timesheet mk-project i-begin
/// timesheet rm-project i-begin
/// timesheet query range midas <start yyyy-mm-dd> (optional, default NOW) <end yyyy-mm-dd> (optional) --as-csv (returns total hours worked)
/// timesheet query two-weeks midas

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IndexFile {
    projects: Vec<Project>,
}

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

    let index_map = load_project_index(Path::new("/home/gabriel/timesheets/index.json"));

    println!("{:?}", index_map);

    match &args.command {
        Subcommands::Clock { project_name } => {}

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
fn load_project_index(path: &Path) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut project_map: HashMap<String, String> = HashMap::new();

    // If index path file does not exist, create it.
    if !&path
        .try_exists()
        .expect("Index path is indeterminate. You may not have read permission.")
    {
        eprint!(
            "Project index path not found. Creating index file at {}",
            Some(&path).unwrap().display()
        );

        let initial_index = IndexFile {
            projects: Vec::new(),
        };
        let initial_json = serde_json::to_string_pretty(&initial_index)
            .expect("Failed to serialize initial index structure.");

        let mut file =
            File::create(path).expect("Creating project index path failed. Path is invalid.");
        file.write_all(initial_json.as_bytes())
            .expect("Failed to write initial JSON to file.");
    }

    // Then read the index file, it will be empty if it was not found in the previous section
    let data = std::fs::read_to_string(&path).expect("Could not load index path.");
    let index: IndexFile = serde_json::from_str(&data).expect("Could not read index file JSON.");

    for project in index.projects {
        project_map.insert(project.name, project.path);
    }

    Ok(project_map)
}

fn load_project(path: &Path) {
    todo!();
}

fn clock_in(project: &Project) {
    todo!();
}
