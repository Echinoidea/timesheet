use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::{collections::HashMap, fs::File, io::Write, path::Path};

/// Project name is self explanatory.
/// Project path is the path to the Timesheet JSON file.
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectIndex {
    name: String,
    path: String,
}

/// This stores a list of projects. This is serialized to JSON.
#[derive(Serialize, Deserialize, Debug)]
pub struct IndexFile {
    project_indexes: Vec<ProjectIndex>,
}

/// Attempt to load the project index JSON file. If it doesn't exist, initialize a new one.
pub fn load_timesheet_index(
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

pub fn serialize_index_hashmap(hashmap: HashMap<String, String>, path: &Path) {
    // Convert HashMap into Vec<ProjectIndex>
    let project_indexes: Vec<ProjectIndex> = hashmap
        .into_iter()
        .map(|(name, path)| ProjectIndex { name, path })
        .collect();

    // Wrap in the top-level struct
    let data = IndexFile { project_indexes };

    // Serialize to JSON
    let json_output = to_string_pretty(&data).unwrap();

    let mut file =
        File::create(path).expect("Creating project index path failed. Path is invalid.");

    file.write_all(json_output.as_bytes())
        .expect("Failed to write initial JSON to file.");
}
