use crate::progress::ProgressTask;
use crate::project_config::{project_name_error, save_config};
use crate::project_config_database::{load_project_database, save_project_to_database};
use crate::shared_types::ProjectConfig;
use crate::shared_types::{
    ESP_FOLDER_NAME, FIRMWARE_DEPENDENCY_LIST, FIRMWARE_TEMPLATE_LIST, LogType,
    SHADCN_COMPONENT_LIST, TAURI_DEPENDENCY_LIST, UI_DEPENDENCY_LIST, UI_FOLDER_NAME,
    UI_TEMPLATE_LIST,
};
use crate::utility::{
    add_dependency, add_node_dependencies, add_shadcn_components, download_file, log,
    node_dependency_steps,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

pub async fn pre_create(input: &Vec<String>) {
    // validate name, prepare directory, firmware, ui, save config
    let mut task = ProgressTask::start("create", 5, "Creating project");

    let Some(project_name) = input.get(2).cloned() else {
        task.fail("Missing project name. Usage: esp create <name> [--path <dir>]");
        return;
    };

    task.step_with("Validating project name", &project_name);
    if let Some(reason) = project_name_error(&project_name) {
        task.fail(format!("Invalid project name '{}': {}", project_name, reason));
        return;
    }

    let existing = load_project_database();
    if existing.iter().any(|p| p.project_name == project_name) {
        task.fail(format!(
            "A project named '{}' is already in the database. Pick a different name.",
            project_name
        ));
        return;
    }

    let target_dir = if input.len() >= 5 && input[3] == "--path" {
        let custom_path = PathBuf::from(&input[4]);
        if !custom_path.is_dir() {
            task.fail(format!(
                "--path {} does not exist or is not a directory",
                custom_path.display()
            ));
            return;
        }
        custom_path
    } else {
        match std::env::current_dir() {
            Ok(dir) => dir,
            Err(error) => {
                task.fail(format!("Could not read the current directory: {}", error));
                return;
            }
        }
    };

    let root_dir = target_dir.join(&project_name);
    task.step_with("Preparing project directory", root_dir.display().to_string());
    if root_dir.exists() {
        log(
            &format!("{} already exists, reusing it", root_dir.display()),
            "Create",
            LogType::Warning,
        );
    }
    if let Err(error) = fs::create_dir_all(&root_dir) {
        task.fail(format!(
            "Could not create {}: {}",
            root_dir.display(),
            error
        ));
        return;
    }

    task.step("Setting up firmware");
    let Some(mut config) = create_firmware(&root_dir, &project_name).await else {
        task.fail(format!("Firmware setup failed, so '{}' was not created", project_name));
        return;
    };

    task.step("Setting up UI");
    if !create_ui(&root_dir, &mut config).await {
        task.fail(format!("UI setup failed, so '{}' is incomplete", project_name));
        return;
    }

    task.step_with("Saving project config", ".espConfig/esp_config.json");
    if save_config(&root_dir, &config).is_none() {
        task.fail("Could not write the project config file");
        return;
    }
    save_project_to_database(&config);

    task.Complete(format!(
        "Project '{}' is ready at {}",
        config.project_name,
        root_dir.display()
    ));
}

/// Scaffolds the ESP firmware crate: cargo generate, then the template files, then the crates.
async fn create_firmware(root_dir: &Path, project_name: &str) -> Option<ProjectConfig> {
    let firmware_dir = root_dir.join(ESP_FOLDER_NAME);
    let total = 1 + FIRMWARE_TEMPLATE_LIST.len() as u32 + FIRMWARE_DEPENDENCY_LIST.len() as u32;
    let mut task = ProgressTask::start(
        "firmware",
        total,
        format!("Setting up firmware in {}", firmware_dir.display()),
    );

    if firmware_dir.exists() {
        task.fail(format!(
            "Firmware directory already exists: {}",
            firmware_dir.display()
        ));
        return None;
    }

    task.step_with(
        "Generating ESP-IDF project",
        "cargo generate esp-rs/esp-idf-template",
    );
    let status = Command::new("cargo")
        .current_dir(root_dir)
        .arg("generate")
        .arg("esp-rs/esp-idf-template")
        .arg("cargo")
        .arg("--name")
        .arg(ESP_FOLDER_NAME)
        .arg("-d")
        .arg("mcu=esp32")
        .arg("-d")
        .arg("advanced=false")
        .status();

    match status {
        Ok(status) if status.success() => {}
        Ok(status) => {
            task.fail(format!("cargo generate failed ({})", status));
            return None;
        }
        Err(error) => {
            task.fail(format!("Could not run cargo generate: {}", error));
            return None;
        }
    }

    for file in FIRMWARE_TEMPLATE_LIST.iter() {
        task.step_with("Downloading firmware template", file.name);
        let output_path = firmware_dir.join(file.output_path);
        if let Err(error) = download_file(file.source_path, &output_path).await {
            task.fail(format!("Could not download {}: {}", file.name, error));
            return None;
        }
    }

    let firmware_path = firmware_dir.display().to_string();
    for dep in FIRMWARE_DEPENDENCY_LIST.iter() {
        if let Err(error) = add_dependency(&firmware_path, dep, &mut task) {
            task.fail(error.to_string());
            return None;
        }
    }

    task.finish(format!("Firmware ready at {}", firmware_dir.display()));

    let mut temp_config = ProjectConfig {
        project_name: project_name.to_string(),
        firmware_path,
        ui_path: String::new(),
        id: Uuid::new_v4().to_string(),
        build_command: "source ~/export-esp.sh && cargo build".to_string(),
        flash_command: "cargo flash --monitor --port /dev/ttyUSB0".to_string(),
        install_components: Vec::new(),
    };
    temp_config.install_components.push("led".to_string());
    temp_config.install_components.push("button".to_string());

    Some(temp_config)
}

/// Scaffolds the Tauri UI: bun create, then the Rust side crates, the template files, the node
/// packages and the shadcn components.
pub async fn create_ui(root_dir: &Path, config: &mut ProjectConfig) -> bool {
    let ui_dir = root_dir.join(UI_FOLDER_NAME);
    let total = 1
        + TAURI_DEPENDENCY_LIST.len() as u32
        + UI_TEMPLATE_LIST.len() as u32
        + node_dependency_steps(&UI_DEPENDENCY_LIST)
        + 1;
    let mut task = ProgressTask::start(
        "ui",
        total,
        format!("Setting up UI in {}", ui_dir.display()),
    );

    task.step_with("Creating Tauri app", "bun create tauri-app --template react-ts");
    let status = Command::new("bun")
        .current_dir(root_dir)
        .arg("create")
        .arg("tauri-app")
        .arg(UI_FOLDER_NAME)
        .arg("--template")
        .arg("react-ts")
        .arg("--manager")
        .arg("bun")
        .arg("--yes")
        .status();

    match status {
        Ok(status) if status.success() => {}
        Ok(status) => {
            task.fail(format!("bun create tauri-app failed ({})", status));
            return false;
        }
        Err(error) => {
            task.fail(format!("Could not run bun create tauri-app: {}", error));
            return false;
        }
    }

    let src_tauri_path = ui_dir.join("src-tauri").display().to_string();
    for dep in TAURI_DEPENDENCY_LIST.iter() {
        if let Err(error) = add_dependency(&src_tauri_path, dep, &mut task) {
            task.fail(error.to_string());
            return false;
        }
    }

    for file in UI_TEMPLATE_LIST.iter() {
        task.step_with("Downloading UI template", file.name);
        let output_path = ui_dir.join(file.output_path.trim_start_matches('/'));

        if let Err(error) = download_file(file.source_path, &output_path).await {
            task.fail(format!("Could not download {}: {}", file.name, error));
            return false;
        }
        if !tokio::fs::try_exists(&output_path).await.unwrap_or(false) {
            task.fail(format!("Template file '{}' was not written", file.name));
            return false;
        }
    }

    if let Err(error) = add_node_dependencies(&ui_dir, &UI_DEPENDENCY_LIST, &mut task) {
        task.fail(error.to_string());
        return false;
    }

    if let Err(error) = add_shadcn_components(&ui_dir, &SHADCN_COMPONENT_LIST, &mut task) {
        task.fail(error.to_string());
        return false;
    }

    config.ui_path = ui_dir.display().to_string();
    task.finish(format!("UI ready at {}", ui_dir.display()));
    true
}
