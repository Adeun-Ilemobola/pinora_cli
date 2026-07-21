use crate::{global_definition::{LogType, ProjectConfig}, utility::log};
use std::path::PathBuf;

fn database_path() -> Option<PathBuf> {
    Some(dirs::home_dir()?.join("esp_rust_projects.json"))
}

/// Every known project. An unreadable or corrupt database is reported and treated as empty
/// rather than panicking mid-command.
pub fn load_project_database() -> Vec<ProjectConfig> {
    let Some(db_path) = database_path() else {
        log("Could not find your home directory.", "Project Database", LogType::Warning);
        return Vec::new();
    };

    if !db_path.exists() {
        return Vec::new();
    }

    let Ok(contents) = std::fs::read_to_string(&db_path) else {
        log(
            &format!("Could not read {}.", db_path.display()),
            "Project Database",
            LogType::Warning,
        );
        return Vec::new();
    };

    serde_json::from_str(&contents).unwrap_or_else(|error| {
        log(
            &format!("Could not parse {}: {}", db_path.display(), error),
            "Project Database",
            LogType::Warning,
        );
        Vec::new()
    })
}

fn write_database(projects: &[ProjectConfig]) {
    let Some(db_path) = database_path() else {
        return;
    };
    let Ok(serialised) = serde_json::to_string_pretty(projects) else {
        return;
    };

    if let Err(error) = std::fs::write(&db_path, serialised) {
        log(
            &format!("Could not write {}: {}", db_path.display(), error),
            "Project Database",
            LogType::Error,
        );
    }
}

pub fn save_project_to_database(project_config: &ProjectConfig) {
    let mut projects = load_project_database();
    projects.push(project_config.clone());
    write_database(&projects);
}

pub fn update_project_config(new_config: &ProjectConfig) {
    let projects: Vec<ProjectConfig> = load_project_database()
        .into_iter()
        .map(|project| {
            if project.id == new_config.id {
                new_config.clone()
            } else {
                project
            }
        })
        .collect();

    write_database(&projects);
}
