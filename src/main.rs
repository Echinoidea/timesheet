use clap::{Parser, Subcommand};

/// Command usage should look like this
/// timesheet clock midas (for clock in if closed)
/// timesheet clock midas (for clock out if open)
/// timesheet mk-project i-begin
/// timesheet rm-project i-begin
/// timesheet query range midas <start yyyy-mm-dd> (optional, default NOW) <end yyyy-mm-dd> (optional) --as-csv (returns total hours worked)
/// timesheet query two-weeks midas

struct Project {
    name: String,
}

//#[derive(Subcommand)]
//enum SubcommandsProject {
//    New { project_name: String },
//    Remove { project_name: String },
//}
//
//#
//enum SubcommandsQuery {
//
//}

#[derive(Subcommand)]
enum Subcommands {
    Clock {
        project_name: String,
    },
    Project {
        command: String,
        project_name: String,
    },
    Query,
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
    println!("Hello, world!");
}
