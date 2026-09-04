use crate::firmware::firmware_definition::ESP_FOLDER_NAME;
use crate::firmware::firmware_store::FIRMWARE_TEMPLATE_LIST;
use crate::global_definition::{LogType, ProjectConfig};
use crate::progress::ProgressTask;
use crate::project_config::project_name_error;
use crate::project_config_database::{create_file_config, load_project_database, save_project_to_database};
use crate::root::NEW_ROOT_TEMPLATE_LIST;
use crate::ui::ui_definition::UI_FOLDER_NAME;
use crate::ui::ui_store::UI_TEMPLATE_LIST;
use crate::utility::{generate_file, log};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;
#[derive(Debug)]
pub  enum FolderError {
    TemplateNotFound(String),
    CouldNotCreateDirectory(String),
    ExecutionFailed(String),

}

#[derive(Debug)]
pub  enum CreateError {
    InvalidProjectName(String),
    ProjectAlreadyExists(String),
    PathDoesNotExist(String),
    CouldNotCreateDirectory(String),
    BuildFailed,
    ExecutionFailed(String),
    Ui(FolderError),
    Firmware(FolderError),
    Root(FolderError),
}


pub async fn pre_create(input: &Vec<String>) -> Result<(), CreateError> {
    // validate name, prepare directory, firmware, ui, save config
    let mut task = ProgressTask::start("create", 5, "Creating project");

    let Some(project_name) = input.get(2).cloned() else {
        task.fail("Missing project name. Usage: esp create <name> [--path <dir>]");
        return Err(CreateError::InvalidProjectName("Missing project name".to_string()));
    };

    task.step_with("Validating project name", &project_name);
    if let Some(reason) = project_name_error(&project_name) {
        task.fail(format!(
            "Invalid project name '{}': {}",
            project_name, reason
        ));
        return Err(CreateError::InvalidProjectName(reason.to_string()));
    }

    let existing = load_project_database();

    if existing.iter().any(|p| p.project_name == project_name) {
        task.fail(format!(
            "A project named '{}' is already in the database. Pick a different name.",
            project_name
        ));
        return Err(CreateError::ProjectAlreadyExists(project_name));
    }

    let target_dir = if input.len() >= 5 && input[3] == "--path" {
        let custom_path = PathBuf::from(&input[4]);
        if !custom_path.is_dir() {
            task.fail(format!(
                "--path {} does not exist or is not a directory",
                custom_path.display()
            ));
            return Err(CreateError::PathDoesNotExist(input[4].clone()));
        }
        custom_path
    } else {
        match std::env::current_dir() {
            Ok(dir) => dir,
            Err(error) => {
                task.fail(format!("Could not read the current directory: {}", error));
                return Err(CreateError::PathDoesNotExist(error.to_string()));
            }
        }
    };

    let root_dir = target_dir.join(&project_name);

    task.step_with(
        "Preparing project directory",
        root_dir.display().to_string(),
    );
    if root_dir.exists() {
        log(
            &format!("{} already exists, reusing it", root_dir.display()),
            "Create",
            LogType::Warning,
        );
        return Err(CreateError::ProjectAlreadyExists(project_name));
    }
    if let Err(error) = fs::create_dir_all(&root_dir) {
        task.fail(format!(
            "Could not create {}: {}",
            root_dir.display(),
            error
        ));
        return Err(CreateError::CouldNotCreateDirectory(root_dir.display().to_string()));
    }

    let new_congif = match create_root_files(&root_dir, &project_name).await {
        Ok(data) => data,
        Err(_s) => return Err(CreateError::ExecutionFailed("Failed to create root files".to_string())),
    };

    let _ = create_firmware(&root_dir, &new_congif).await;

    let _ = create_ui(&root_dir, &new_congif).await;

    let status = Command::new("just")
        .arg("build-all")
        .current_dir(&root_dir)
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("Build succeeded: {status}");
            task.finish("Build succeeded");
            let _ = save_project_to_database(&new_congif);
            return Ok(());
        }
        Ok(status) => {
            eprintln!("Build failed: {status}");
            task.fail("Build failed");
            return Err(CreateError::BuildFailed);
        }
        Err(error) => {
            eprintln!("Failed to execute `just`: {error}");
            task.fail("Failed to execute `just`");
            return Err(CreateError::ExecutionFailed("Failed to execute `just`".to_string()));
        }
    }

  
}

async fn create_root_files(root_dir: &Path, project_name: &str) -> Result<ProjectConfig, CreateError> {
    let temp_config = ProjectConfig {
        project_name: project_name.to_string(),
        firmware_path: format!("{}", root_dir.join(ESP_FOLDER_NAME).display()),
        ui_path: format!("{}", root_dir.join(UI_FOLDER_NAME).display()),
        id: Uuid::new_v4().to_string(),
        build_command: "just frontend".to_string(),
        flash_command: "just flash".to_string(),
        install_components: Vec::new(),
    };

    match create_file_config(&temp_config, &root_dir.to_path_buf()) {
        Ok(_) => {}
        Err(error) => return Err(CreateError::ExecutionFailed(format!("Failed to create file config: {:?}", error))),
    }

    for item in NEW_ROOT_TEMPLATE_LIST.iter() {
        match generate_file(item, root_dir, &temp_config).await {
            Ok(_) => {}
            Err(err) => return Err(CreateError::Root(FolderError::CouldNotCreateDirectory(
                format!("{:?}", err),
            ))),
        }
    }

    Ok(temp_config)
}

async fn create_firmware(root_dir: &Path, temp_config: &ProjectConfig) -> Result<(), CreateError> {
    let firmware_path = root_dir.join(ESP_FOLDER_NAME);
    let _ = fs::create_dir_all(&firmware_path);

    for item in FIRMWARE_TEMPLATE_LIST.iter() {
        match generate_file(item, &firmware_path.as_path(), &temp_config).await {
            Ok(_) => {}
            Err(err) => return Err(CreateError::Firmware(FolderError::CouldNotCreateDirectory(
                format!("{:?}", err),
            ))),
        }
    }
    Ok(())
}

async fn create_ui(root_dir: &Path, temp_config: &ProjectConfig) -> Result<(), CreateError> {
    let ui_path = root_dir.join(UI_FOLDER_NAME);
    let _ = fs::create_dir_all(&ui_path);

    for item in UI_TEMPLATE_LIST.iter() {
        match generate_file(item, &ui_path.as_path(), &temp_config).await {
            Ok(_) => {}
            Err(err) => return Err(CreateError::Ui(FolderError::CouldNotCreateDirectory(
                format!("{:?}", err),
            ))),
        }
    }
    Ok(())
}
