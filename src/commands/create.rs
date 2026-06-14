
use crate::sharedTypes::{  LogType, ProjectConfig };
use crate::projectConfigDatabase::{load_project_database , update_project_struct , save_project_to_database};
use crate::utility::{emit_progress , log  };
use crate::projectConfig::{ create_config , project_name_is_valid};
use std::process::Command;
use uuid::Uuid;


async fn create_project(
    current_dir: &std::path::PathBuf,
    project_name: &str,
) -> Option<ProjectConfig> {
    let total_steps = 99;
    emit_progress(
        "Project Creation",
        "Validating project name...",
        1,
        total_steps,
    );
    let project_dir = current_dir.join(&project_name);
    // let build_script = "source ~/export-esp.sh && cargo build";

    if project_dir.exists() {
        log(
            &format!(
                "Project directory already exists at: {}",
                project_dir.display()
            ),
            "Project Creation",
            LogType::Warning,
        );
        return None;
    }
    emit_progress(
        "Project Creation",
        "Generating project files...",
        2,
        total_steps,
    );
    // (2) run "cargo generate esp-rs/esp-idf-template {project_name}"
    let gen_status = Command::new("cargo")
        .current_dir(&current_dir)
        .arg("generate")
        .arg("esp-rs/esp-idf-template")
        .arg("cargo")
        .arg("--name")
        .arg(&project_name)
        .arg("-d")
        .arg("mcu=esp32")
        .arg("-d")
        .arg("advanced=false")
        .status()
        .expect("Failed to run cargo generate");

    if gen_status.success() {
        emit_progress(
            "Project Creation",
            "Creating project config file...",
            3,
            total_steps,
        );
        let project_identifier = create_config(
            &project_dir,
            &ProjectConfig {
                project_name: project_name.to_string(),
                project_path: project_dir.display().to_string(),
                id: Uuid::new_v4().to_string(),
                build_command: "source ~/export-esp.sh && cargo build".to_string(),
                flash_command: "cargo flash --monitor --port /dev/ttyUSB0".to_string(),
                install_components: Vec::new(),
            },
        );

        emit_progress(
            "Project Creation",
            "Creating project config file...",
            4,
            total_steps,
        );
        if project_identifier.is_some() {
            let config_data = project_identifier.as_ref().unwrap();
            log(
                "Project config file created successfully!",
                "Project Config Creation",
                LogType::Complete,
            );

            if update_project_struct(&project_dir, total_steps).await {
                save_project_to_database(&config_data);
                emit_progress(
                    "Project Creation",
                    "Project creation completed.",
                    total_steps,
                    total_steps,
                );
                return Some(config_data.clone());
            } else {
                log(
                    "Failed to update project structure.",
                    "Project Structure Update",
                    LogType::Error,
                );
                return None;
            }
        } else {
            log(
                "Failed to create project config file.",
                "Project Config Creation",
                LogType::Error,
            );
            return None;
        }
    } else {
        log(
            "Failed to generate project.",
            "Project Generation",
            LogType::Error,
        );
        return None;
    }
}


pub async fn pre_create(input:&Vec<String>) {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    if input.len() < 3 {
        log(
            "Please provide a project name.",
            "Command Validation",
            LogType::Error,
        );
        return;
    }
    let project_name = input[2].clone();

    let all_projects = load_project_database().expect("Failed to load project database");
    if all_projects.iter().any(|p| p.project_name == project_name) {
        log(
            "A project with this name already exists in the database. Please choose a different name.",
            "Project Name Validation",
            LogType::Error,
        );
        return;
    }

    if !project_name_is_valid(&project_name) {
        return;
    }
    if input.len() >= 5 && input[3] == "--path" {
        let custom_path = std::path::PathBuf::from(&input[4]);
        log(
            &format!("the path: {}", custom_path.display()),
            "Path Validation",
            LogType::Info,
        );
        if !custom_path.exists() || !custom_path.is_dir() {
            log(
                "Provided path does not exist or is not a directory.",
                "Path Validation",
                LogType::Error,
            );
            return;
        }
        let project_path = custom_path.join(&project_name);
        if project_path.exists() {
            log(
                "Project already exists at the provided path.",
                "Project Creation",
                LogType::Error,
            );
            return;
        }
        match create_project(&custom_path, &project_name).await {
            Some(config_data) => {
                log(
                    &format!(
                        "Project '{}' created successfully at {}!",
                        config_data.project_name,
                        project_path.display()
                    ),
                    "Project Creation",
                    LogType::Info,
                );
            }
            None => {
                log(
                    "Failed to create project.",
                    "Project Creation",
                    LogType::Error,
                );
            }
        }
    } else if input.len() >= 3 {
        log(
            &format!("the path: {}", current_dir.display()),
            "Path Validation",
            LogType::Info,
        );
        match create_project(&current_dir, &project_name).await {
            Some(config_data) => {
                log(
                    &format!(
                        "Project '{}' created successfully at {}!",
                        config_data.project_name,
                        current_dir.join(&config_data.project_name).display()
                    ),
                    "Project Creation",
                    LogType::Info,
                );
            }
            None => {
                log(
                    "Failed to create project.",
                    "Project Creation",
                    LogType::Error,
                );
            }
        }
    } else {
        log(
            "Invalid command format. Please provide a project name and optionally a path.",
            "Command Validation",
            LogType::Error,
        );
        log(
            "Example: project create my_project --path /path/to/projects",
            "Command Validation",
            LogType::Info,
        );
        return;
    }
}
