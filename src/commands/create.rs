use crate::file_json::load_files_json;
use crate::projectConfig::{create_config, project_name_is_valid, save_config};
use crate::projectConfigDatabase::{load_project_database, save_project_to_database};
use crate::sharedTypes::{ESP_FOLDER_NAME, FIRMWARE_TEMPLATE_LIST, TemplateFile};
use crate::sharedTypes::{LogType, ProgressType, ProjectConfig};
use crate::utility::{download_file, log, progress_log};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;

const CREATE_ID: &str = "create-project";
const BILD_STRUCT: &str = "update_project_struct";

async fn create_project(
    current_dir: &std::path::PathBuf,
    root_folder_name: &String,
) -> Option<ProjectConfig> {
    let project_dir = current_dir.join(&ESP_FOLDER_NAME);

    progress_log(
        ProgressType::Loading,
        format!("Checking project directory for '{}'", ESP_FOLDER_NAME),
        CREATE_ID.to_string(),
    );

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

    progress_log(
        ProgressType::Loading,
        format!("Running cargo generate for '{}'", ESP_FOLDER_NAME),
        CREATE_ID.to_string(),
    );

    let gen_status = Command::new("cargo")
        .current_dir(&current_dir)
        .arg("generate")
        .arg("esp-rs/esp-idf-template")
        .arg("cargo")
        .arg("--name")
        .arg(&ESP_FOLDER_NAME)
        .arg("-d")
        .arg("mcu=esp32")
        .arg("-d")
        .arg("advanced=false")
        .status()
        .expect("Failed to run cargo generate");

    if !gen_status.success() {
        log(
            "Failed to generate project.",
            "Project Generation",
            LogType::Error,
        );
        return None;
    }

    progress_log(
        ProgressType::Loading,
        "Writing project config file (.espConfig/esp_config.json)".to_string(),
        CREATE_ID.to_string(),
    );

    let config_data = ProjectConfig {
        project_name: root_folder_name.clone(),
        ui_path: String::new(),
        firmware_path: project_dir.display().to_string(),
        id: Uuid::new_v4().to_string(),
        build_command: "source ~/export-esp.sh && cargo build".to_string(),
        flash_command: "cargo flash --monitor --port /dev/ttyUSB0".to_string(),
        install_components: Vec::new(),
    };

    progress_log(
        ProgressType::Loading,
        "Updating project structure (downloading template files)".to_string(),
        CREATE_ID.to_string(),
    );

    if !update_project_struct(&project_dir).await {
        log(
            "Failed to update project structure.",
            "Project Structure Update",
            LogType::Error,
        );
        return None;
    }

    Some(config_data)
}

pub async fn pre_create(input: &Vec<String>) {
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

    progress_log(
        ProgressType::Loading,
        format!("Validating project name '{}'", project_name),
        CREATE_ID.to_string(),
    );

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

    let target_dir = if input.len() >= 5 && input[3] == "--path" {
        let custom_path = std::path::PathBuf::from(&input[4]);
        if !custom_path.exists() || !custom_path.is_dir() {
            log(
                "Provided path does not exist or is not a directory.",
                "Path Validation",
                LogType::Error,
            );
            return;
        }
        custom_path
    } else {
        current_dir
    };

    let make_root_folder = target_dir.join(&project_name);
    let _ = fs::create_dir(&make_root_folder);

    progress_log(
        ProgressType::Loading,
        format!(
            "Creating project '{}' at {}",
            &project_name,
            make_root_folder.display()
        ),
        CREATE_ID.to_string(),
    );

    match create_project(&make_root_folder, &project_name).await {
        Some(config_data) => {
            save_project_to_database(&config_data);
            save_config(&make_root_folder, &config_data);

            progress_log(
                ProgressType::Complete,
                "Project config file created".to_string(),
                CREATE_ID.to_string(),
            );

            progress_log(
                ProgressType::Finished,
                format!(
                    "Project '{}' created at {}",
                    project_name.clone(),
                    config_data.firmware_path
                ),
                CREATE_ID.to_string(),
            );

            log(
                &format!(
                    "Project '{}' created successfully at {}!",
                    config_data.project_name,
                    target_dir.join(&config_data.project_name).display()
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
}

async fn update_project_struct(project_path: &std::path::PathBuf) -> bool {
    for file in &FIRMWARE_TEMPLATE_LIST {
        progress_log(
            ProgressType::Loading,
            format!("Downloading: {}", file.name),
            BILD_STRUCT.to_string(),
        );
        let output_path = project_path.join(&file.output_path);

        match download_file(&file.source_path, &output_path).await {
            Ok(_) => progress_log(
                ProgressType::Complete,
                format!("Downloaded: {}", file.name),
                BILD_STRUCT.to_string(),
            ),
            Err(e) => {
                progress_log(
                    ProgressType::Error,
                    format!("Failed to download {}: {}", output_path.display(), e),
                    BILD_STRUCT.to_string(),
                );
                log(
                    &format!("Failed to create file {}: {}", output_path.display(), e),
                    "Project Structure Update",
                    LogType::Error,
                );
            }
        }
    }

    let dependency_to_add_list = vec![
        ("uuid", None),
        ("serde", Some("derive")),
        ("serde_json", None),
        ("anyhow", None),
    ];

    progress_log(
        ProgressType::Installing,
        format!(
            "Installing {} dependencies...",
            dependency_to_add_list.len()
        ),
        BILD_STRUCT.to_string(),
    );

    for (dep, feature) in dependency_to_add_list {
        progress_log(
            ProgressType::Installing,
            format!("Adding dependency '{}'...", dep),
            BILD_STRUCT.to_string(),
        );
        let mut cmd = Command::new("cargo");
        cmd.arg("add").arg(dep).current_dir(&project_path);
        if let Some(feature_name) = feature {
            cmd.arg("--features").arg(feature_name);
        }
        match cmd.status() {
            Ok(status) if status.success() => progress_log(
                ProgressType::Complete,
                format!("Dependency '{}' added.", dep),
                BILD_STRUCT.to_string(),
            ),
            Ok(status) => progress_log(
                ProgressType::Error,
                format!("Failed to add '{}'. Exit status: {}", dep, status),
                BILD_STRUCT.to_string(),
            ),
            Err(e) => progress_log(
                ProgressType::Error,
                format!("Failed to run cargo add for '{}': {}", dep, e),
                BILD_STRUCT.to_string(),
            ),
        };
    }

    progress_log(
        ProgressType::Installing,
        "Patching Cargo.toml with uuid version constraint...".to_string(),
        BILD_STRUCT.to_string(),
    );

    let cargo_toml_path = project_path.join("Cargo.toml");
    if cargo_toml_path.exists() {
        let mut cargo_toml_content =
            std::fs::read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml");
        let marker = "[dependencies]";
        let dependency_to_add = r#"uuid = { version = "1.23.2", features = ["v4"] }"#;
        if let Some(index) = cargo_toml_content.find(marker) {
            let old_dependency_line = cargo_toml_content
                .lines()
                .find(|line| line.contains(r#"uuid = "1.20.0"#));
            if let Some(old_line) = old_dependency_line {
                cargo_toml_content = cargo_toml_content.replace(old_line, "");
            }

            let insert_position = index + marker.len();
            cargo_toml_content.insert_str(insert_position, &format!("\n{}", dependency_to_add));
        }

        std::fs::write(&cargo_toml_path, cargo_toml_content).expect("Failed to update Cargo.toml");
        progress_log(
            ProgressType::Complete,
            "Cargo.toml patched with uuid dependency.".to_string(),
            BILD_STRUCT.to_string(),
        );
    }

    progress_log(
        ProgressType::Finished,
        "Project structure update complete.".to_string(),
        BILD_STRUCT.to_string(),
    );

    return true;
}
