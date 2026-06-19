use crate::utility::{download_file, emit_progress, log};
use crate::sharedTypes::{ProjectConfig, LogType , TemplateFile};
use crate::file_json::load_files_json;
use std::path::{ PathBuf};
use std::process::Command;


pub fn load_project_database() -> Option<Vec<ProjectConfig>> {
    let projects: Vec<ProjectConfig> = Vec::new();

    let home_dir = dirs::home_dir().expect("Failed to get home directory");

    let db_path = home_dir.join("esp_rust_projects.json");

    if !db_path.exists() {
        log(
            "Project database file does not exist. Creating a new one.",
            "Project Database",
            LogType::Warning,
        );
        std::fs::write(&db_path, "[]").expect("Failed to create project database file");
        return Some(projects);
    }
    let db_content =
        std::fs::read_to_string(&db_path).expect("Failed to read project database file");
    let parsed_projects: Vec<ProjectConfig> =
        serde_json::from_str(&db_content).expect("Failed to parse project database file");
    Some(parsed_projects)
}

pub fn save_project_to_database(project_config: &ProjectConfig) {
    let mut projects = load_project_database().expect("Failed to load project database");
    projects.push(project_config.clone());
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let db_path = home_dir.join("esp_rust_projects.json");
    std::fs::write(
        &db_path,
        serde_json::to_string_pretty(&projects).expect("Failed to serialize project database"),
    )
    .expect("Failed to write project database file");
    log(
        "Project saved to database successfully!",
        "Project Database",
        LogType::Info,
    );
}



pub  fn update_project_config(new_config: &ProjectConfig){
    let get_projects = load_project_database().expect("Failed to load project database");
    let  updated_projects: Vec<ProjectConfig> = get_projects.into_iter()
        .map(|proj| if proj.id == new_config.id { new_config.clone() } else { proj })
        .collect();
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let db_path = home_dir.join("esp_rust_projects.json");
    std::fs::write(
        &db_path,
        serde_json::to_string_pretty(&updated_projects).expect("Failed to serialize project database"),
    )
    .expect("Failed to write project database file");
    log(
        "Project saved to database successfully!",
        "Project Database",
        LogType::Info,
    );
}

