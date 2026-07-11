use crate::project_config::{ project_name_is_valid, save_config};
use crate::project_config_database::{load_project_database, save_project_to_database};
use crate::sharedtypes::{
    ESP_FOLDER_NAME, FIRMWARE_DEPENDENCY_LIST, FIRMWARE_TEMPLATE_LIST, SHADCN_COMPONENT_LIST,
    TAURI_DEPENDENCY_LIST, UI_DEPENDENCY_LIST, UI_FOLDER_NAME, UI_TEMPLATE_LIST,
};
use crate::sharedtypes::{LogType, ProgressType, ProjectConfig};
use crate::utility::{
    add_dependency, add_node_dependencies, add_shadcn_components, download_file, log, progress_log,
};
use std::fs;
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
            let mut new_config = config_data;
            let _ui_success = create_ui(&make_root_folder, &mut new_config).await;
            save_project_to_database(&new_config);
            save_config(&make_root_folder, &new_config);

            progress_log(
                ProgressType::Complete,
                "Project config file created".to_string(),
                CREATE_ID.to_string(),
            );

            progress_log(
                ProgressType::Finished,
                format!(
                    "Project '{}' created at {}",
                    new_config.project_name, new_config.firmware_path
                ),
                CREATE_ID.to_string(),
            );

            log(
                &format!(
                    "Project '{}' created successfully at {}!",
                    new_config.project_name,
                    target_dir.join(&new_config.project_name).display()
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

    for dep in FIRMWARE_DEPENDENCY_LIST.iter() {
        if let Err(e) = add_dependency(project_path.to_str().unwrap_or_default(), dep, BILD_STRUCT).await {
            progress_log(
                ProgressType::Error,
                format!("Failed to run cargo add for '{}': {}", dep.name, e),
                "pre-build-ui".to_string(),
            );
        }
    }

    progress_log(
        ProgressType::Installing,
        "Patching Cargo.toml with uuid version constraint...".to_string(),
        BILD_STRUCT.to_string(),
    );

    // let cargo_toml_path = project_path.join("Cargo.toml");
    // if cargo_toml_path.exists() {
    //     let mut cargo_toml_content =
    //         std::fs::read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml");
    //     let marker = "[dependencies]";
    //     let dependency_to_add = r#"uuid = { version = "1.23.2", features = ["v4"] }"#;
    //     if let Some(index) = cargo_toml_content.find(marker) {
    //         let old_dependency_line = cargo_toml_content
    //             .lines()
    //             .find(|line| line.contains(r#"uuid = "1.20.0"#));
    //         if let Some(old_line) = old_dependency_line {
    //             cargo_toml_content = cargo_toml_content.replace(old_line, "");
    //         }

    //         let insert_position = index + marker.len();
    //         cargo_toml_content.insert_str(insert_position, &format!("\n{}", dependency_to_add));
    //     }

    //     std::fs::write(&cargo_toml_path, cargo_toml_content).expect("Failed to update Cargo.toml");
    //     progress_log(
    //         ProgressType::Complete,
    //         "Cargo.toml patched with uuid dependency.".to_string(),
    //         BILD_STRUCT.to_string(),
    //     );
    // }

    progress_log(
        ProgressType::Finished,
        "Project structure update complete.".to_string(),
        BILD_STRUCT.to_string(),
    );

    return true;
}

pub async fn create_ui(project_path: &std::path::PathBuf, config: &mut ProjectConfig) -> bool {
    progress_log(
        ProgressType::Installing,
        "Installing Tauri app with bun...".to_string(),
        "pre-build-ui".to_string(),
    );

    let status = tokio::process::Command::new("bun")
        .arg("create")
        .arg("tauri-app")
        .arg(UI_FOLDER_NAME)
        .arg("--template")
        .arg("react-ts")
        .arg("--manager")
        .arg("bun")
        .arg("--yes")
        .current_dir(project_path)
        .status()
        .await;
    match status {
        Ok(s) if s.success() => {
            let mut has_failed = false;
            progress_log(
                ProgressType::Complete,
                "Tauri app created successfully.".to_string(),
                "pre-build-ui".to_string(),
            );

            let ui_project_path = project_path.join(UI_FOLDER_NAME);
            let src_tauri_path = ui_project_path.join("src-tauri");
            let src_tauri_path = src_tauri_path.to_str().unwrap_or_default();

            progress_log(
                ProgressType::Installing,
                format!("Installing {} dependencies...", TAURI_DEPENDENCY_LIST.len()),
                "pre-build-ui".to_string(),
            );

            for dep in TAURI_DEPENDENCY_LIST.iter() {
                if let Err(e) = add_dependency(src_tauri_path, dep, "pre-build-ui").await {
                    progress_log(
                        ProgressType::Error,
                        format!("Failed to run cargo add for '{}': {}", dep.name, e),
                        "pre-build-ui".to_string(),
                    );
                }
            }
            for file_ob in UI_TEMPLATE_LIST.iter() {
                let output_path = ui_project_path.join(file_ob.output_path.trim_start_matches('/'));
                let _ = download_file(&file_ob.source_path, &output_path).await;

                match tokio::fs::try_exists(&output_path).await {
                    Ok(true) => {
                        progress_log(
                            ProgressType::Complete,
                            format!("Downloaded template file '{}'.", file_ob.name),
                            "pre-build-ui".to_string(),
                        );
                    }
                    _ => {
                        progress_log(
                            ProgressType::Error,
                            format!("Template file '{}' was not created.", file_ob.name),
                            "pre-build-ui".to_string(),
                        );
                        has_failed = true;
                        break;
                    }
                }
            }

            if !has_failed {
                if let Err(e) =
                    add_node_dependencies(&ui_project_path, &UI_DEPENDENCY_LIST, "pre-build-ui")
                        .await
                {
                    log(
                        &format!("Failed to install UI dependencies: {}", e),
                        "UI Setup",
                        LogType::Error,
                    );
                    has_failed = true;
                }
            }

            if !has_failed {
                if let Err(e) =
                    add_shadcn_components(&ui_project_path, &SHADCN_COMPONENT_LIST, "pre-build-ui")
                        .await
                {
                    log(
                        &format!("Failed to add shadcn components: {}", e),
                        "UI Setup",
                        LogType::Error,
                    );
                    has_failed = true;
                }
            }

            progress_log(
                ProgressType::Finished,
                "Tauri dependency installation complete.".to_string(),
                "pre-build-ui".to_string(),
            );

            if has_failed {
                false
            } else {
                config.ui_path = ui_project_path.display().to_string();
                true
            }
        }
        Ok(s) => {
            log(
                &format!("bun create tauri-app failed with exit status: {}", s),
                "UI Setup",
                LogType::Error,
            );
            false
        }
        Err(e) => {
            log(
                &format!("Failed to run bun create tauri-app: {}", e),
                "UI Setup",
                LogType::Error,
            );
            false
        }
    }
}
