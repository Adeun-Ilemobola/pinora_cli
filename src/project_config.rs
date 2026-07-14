use std::path::{Path, PathBuf};

use crate::project_config_database::update_project_config;
use crate::shared_types::{ESP_FOLDER_NAME, ProjectConfig, UI_FOLDER_NAME};

const CONFIG_RELATIVE_PATH: &str = ".espConfig/esp_config.json";

/// Looks for the project config in the current directory, then in the firmware and UI
/// subdirectories, so commands work from anywhere inside a project.
pub fn load_config() -> Option<ProjectConfig> {
    let root_dir = std::env::current_dir().ok()?;

    let candidates: [PathBuf; 3] = [
        root_dir.clone(),
        root_dir.join(ESP_FOLDER_NAME),
        root_dir.join(UI_FOLDER_NAME),
    ];

    for candidate in candidates {
        let config_path = candidate.join(CONFIG_RELATIVE_PATH);
        if !config_path.exists() {
            continue;
        }
        let contents = std::fs::read_to_string(&config_path).ok()?;
        return serde_json::from_str(&contents).ok();
    }

    None
}

pub fn save_config(path: &Path, config: &ProjectConfig) -> Option<ProjectConfig> {
    let config_path = path.join(CONFIG_RELATIVE_PATH);

    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).ok()?;
    }
    let serialised = serde_json::to_string_pretty(config).ok()?;
    std::fs::write(&config_path, serialised).ok()?;

    Some(config.clone())
}

pub fn update_config_file_with_component(project_path: &Path, component_name: &str) -> bool {
    let Some(mut config) = load_config() else {
        return false;
    };

    config.install_components.push(component_name.to_string());

    if save_config(project_path, &config).is_none() {
        return false;
    }

    update_project_config(&config);
    true
}

/// Returns why the name is unusable, or `None` if it is fine.
pub fn project_name_error(project_name: &str) -> Option<&'static str> {
    if project_name.trim().is_empty() {
        return Some("it cannot be empty");
    }
    if project_name.contains(' ') {
        return Some("it cannot contain spaces");
    }
    if project_name.contains('/') || project_name.contains('\\') {
        return Some("it cannot contain path separators");
    }
    if project_name.contains('.') {
        return Some("it cannot contain dots");
    }
    if project_name.contains('-') {
        return Some("it cannot contain hyphens");
    }
    if project_name.len() < 3 {
        return Some("it must be at least 3 characters");
    }
    if project_name.len() > 100 {
        return Some("it must be at most 100 characters");
    }
    None
}
