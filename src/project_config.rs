use std::path::PathBuf;

use crate::project_config_database::update_project_config;
use crate::sharedtypes::{ESP_FOLDER_NAME, LogType, ProjectConfig, UI_FOLDER_NAME};
use crate::utility::log;

pub fn load_config() -> Option<ProjectConfig> {
    let rootdir = &std::env::current_dir().expect("Failed to get current directory");
    let config_path_file = rootdir.join(".espConfig/esp_config.json");

    let mut current_config: Option<ProjectConfig> = None;
    let possible_path: [PathBuf; 3] = [
        rootdir.clone(),
        rootdir.join(&ESP_FOLDER_NAME).clone(),
        rootdir.join(&UI_FOLDER_NAME).clone(),
    ];
    for p in possible_path {
        if p.join(&config_path_file).exists() {
            let config_content = std::fs::read_to_string(&config_path_file)
                .expect("Failed to read project config file");
            let config: ProjectConfig =
                serde_json::from_str(&config_content).expect("Failed to parse project config file");

            current_config = Some(config);
            break;
        }
    }

    return current_config;
}
pub fn save_config(path: &std::path::PathBuf, config: &ProjectConfig) -> Option<ProjectConfig> {
    let config_path_file = path.join(".espConfig/esp_config.json");

    if let Some(parent) = config_path_file.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create config directory");
    }
    std::fs::write(
        &config_path_file,
        serde_json::to_string_pretty(config).expect("Failed to serialize project config"),
    )
    .expect("Failed to write project config file");
    log(
        "Project config file saved successfully!",
        "Project Config Save",
        LogType::Info,
    );
    Some(config.clone())
}
pub fn update_config_file_with_component(
    project_path: &std::path::PathBuf,
    component_name: &str,
) -> bool {
   
    if let Some(mut config) = load_config() {
        let install_components = &mut config.install_components;

        install_components.push(component_name.to_string());

        if save_config(project_path, &config).is_some() {
            log(
                &format!("Component '{}' added to project config.", component_name),
                "Project Config Update",
                LogType::Info,
            );
            update_project_config(&config);
            return true;
        } else {
            log(
                "Failed to find 'install_components' in project config.",
                "Project Config Update",
                LogType::Error,
            );
            return false;
        }
    }
    log(
        "Project config file does not exist.",
        "Project Config Update",
        LogType::Error,
    );
    return false;
}

pub fn _create_config(path: &std::path::PathBuf, config: &ProjectConfig) -> Option<ProjectConfig> {
    let get_config = load_config();

    if get_config.is_none() {
        log(
            "Project config file does not exist. Creating a new one.",
            "--Project Config Creation--",
            LogType::Info,
        );
        let saved_config = save_config(path, config);
        if saved_config.is_some() {
            return saved_config;
        } else {
            log(
                "Failed to create project config file.",
                "--Project Config Creation--",
                LogType::Error,
            );
            return None;
        }
    }
    return None;
}

pub fn project_name_is_valid(project_name: &str) -> bool {
    if project_name.is_empty() || project_name.trim().is_empty() || project_name.contains(' ') {
        log(
            "Project name cannot be empty or contain spaces.",
            "Project Name Validation",
            LogType::Error,
        );
        return false;
    }
    if project_name.contains('/') || project_name.contains('\\') {
        log(
            "Project name cannot contain path separators.",
            "Project Name Validation",
            LogType::Error,
        );
        return false;
    }
    if project_name.contains('.') {
        log(
            "Project name cannot contain dots.",
            "Project Name Validation",
            LogType::Error,
        );
        return false;
    }
    if project_name.contains('-') {
        log(
            "Project name cannot contain hyphens.",
            "Project Name Validation",
            LogType::Error,
        );
        return false;
    }
    if project_name.len() > 100 {
        log(
            "Project name cannot be longer than 100 characters.",
            "Project Name Validation",
            LogType::Error,
        );
        return false;
    }
    if project_name.len() < 3 {
        log(
            "Project name cannot be shorter than 3 characters.",
            "Project Name Validation",
            LogType::Error,
        );
        return false;
    }
    true
}
