use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::{fs::File, io::Write, path::Path};

/// An entry in a project Timesheet.
#[derive(Serialize, Deserialize, Debug)]
pub struct TimesheetEntry {
    time_in: String,
    time_out: String,
    message: String,
}

/// A project Timesheet.
#[derive(Serialize, Deserialize, Debug)]
pub struct Timesheet {
    entries: Vec<TimesheetEntry>,
}

impl Timesheet {
    /// Static function for initializing a timesheet JSON file
    pub fn initialize_timesheet_json(path: &Path) {
        // Initialize JSON structure with no contents
        let initial_timesheet = Timesheet {
            entries: Vec::new(),
        };

        let initial_json = to_string_pretty(&initial_timesheet)
            .expect("Failed to serialize initial index structure.");

        let mut file =
            File::create(path).expect("Creating project index path failed. Path is invalid.");

        // Write the empty formatted JSON
        file.write_all(initial_json.as_bytes())
            .expect("Failed to write initial JSON to file.");
    }

    /// Load a project/timesheet JSON file and load it into a vector of TimesheetEntry
    pub fn load_timesheet(path: &Path) -> Timesheet {
        // If timesheet path file does not exist, create it.
        if !path
            .try_exists()
            .expect("Index path is indeterminate. You may not have read permission.")
        {
            eprintln!("Project index path not found. {}", path.display());
        }

        // Load the data, if it cannot find the path, create it
        let data = match std::fs::read_to_string(&path) {
            Ok(value) => value,
            Err(_) => {
                Self::initialize_timesheet_json(&path);
                std::fs::read_to_string(&path).unwrap()
            }
        };

        let timesheet: Timesheet = serde_json::from_str(&data).expect(
            "Could not read timesheet file JSON. Check to make sure JSON structure is valid.",
        );

        let mut entries: Vec<TimesheetEntry> = vec![];

        for entry in timesheet.entries {
            entries.push(entry);
        }

        Timesheet { entries }
    }

    /// Write timesheet data to JSON
    pub fn serialize(self: &Self, path: &Path) {
        let data = to_string_pretty(&self).expect("Failed to serialize initial index structure.");

        let mut file = File::create(path).expect("Error creating file buffer to write timesheet");

        file.write_all(data.as_bytes())
            .expect("Failed to write initial JSON to file.");
    }

    pub fn is_clocked_in(self: &mut Self) -> bool {
        match self.entries.last() {
            Some(last_entry) => last_entry.time_out.is_empty(),
            None => false,
        }
    }

    pub fn is_clocked_out(self: &mut Self) -> bool {
        !self.is_clocked_in()
    }

    pub fn clock_in(self: &mut Self, message: &String) {
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
                        message: format!("IN: {}", message.to_string()),
                    });
                    println!("Clocked into a new entry.");
                }
            }
            None => {
                // No entries yet, create a new one and clock in
                self.entries.push(TimesheetEntry {
                    time_in: now.clone(),
                    time_out: String::new(),
                    message: format!("IN: {}", message.to_string()),
                });
                println!("Clocked in successfully.");
            }
        }
    }

    pub fn clock_out(self: &mut Self, message: &String) {
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

                    if !message.is_empty() {
                        if last_entry.message.is_empty() {
                            last_entry.message.push_str(format!("{}", message).as_str());
                        } else {
                            last_entry
                                .message
                                .push_str(format!(" | OUT: {}", message).as_str());
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
