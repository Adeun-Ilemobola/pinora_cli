use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio;

use crate::progress::ProgressTask;
use crate::project_config::{load_config, update_config_file_with_component};
use crate::sharedtypes::BRANCH_NAME;
use crate::utility::download_file;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ModuleShape {
    pub name: String,
    pub source: String,
    pub default: bool,
}

pub async fn load_modules() -> Result<Vec<ModuleShape>, anyhow::Error> {
    let modules_data_file = PathBuf::from("moduleDatabase.json");

    let json_text = tokio::fs::read_to_string(&modules_data_file)
        .await
        .context("Failed to read JSON file")?;

    let files: Vec<ModuleShape> =
        serde_json::from_str(&json_text).context("Failed to parse JSON file")?;

    Ok(files)
}

pub async fn add_modules(name: String) -> Result<(), anyhow::Error> {
    let mut task = ProgressTask::start("add module", 8, format!("Installing component '{}'", name));

    let database = match load_modules().await {
        Ok(database) => database,
        Err(error) => {
            task.fail(format!(
                "Could not read the component registry (moduleDatabase.json): {}",
                error
            ));
            return Ok(());
        }
    };
    task.step_with(
        "Loaded component registry",
        format!("{} components available", database.len()),
    );

    let Some(found_module) = database.iter().find(|i| i.name == name) else {
        task.fail(format!(
            "'{}' is not in the component registry. Run `esp listcomponents` to see them all.",
            name
        ));
        return Ok(());
    };
    task.step_with("Found component in registry", &found_module.source);

    let Some(data) = load_config() else {
        task.fail(
            "No project config found here. Run `esp create <name>`, or cd into an existing project.",
        );
        return Ok(());
    };
    task.step_with("Loaded project config", &data.project_name);

    if data.install_components.contains(&found_module.name) {
        task.Complete(format!(
            "'{}' is already installed in '{}', so nothing to do",
            found_module.name, data.project_name
        ));
        return Ok(());
    }
    task.step("Confirmed the component is not installed yet");

    let firmware_path = PathBuf::from(&data.firmware_path);

    let Some(get_project_root) = firmware_path.parent().map(Path::to_path_buf) else {
        task.fail(format!(
            "Firmware path '{}' has no parent directory, so the project root cannot be resolved",
            firmware_path.display()
        ));
        return Ok(());
    };
    task.step_with("Resolved project root", get_project_root.display().to_string());

    let firmware_module_folder = firmware_path.join("src").join("module");

    if !firmware_module_folder.is_dir() {
        task.fail(format!(
            "No module folder at {}. The project layout looks incomplete.",
            firmware_module_folder.display()
        ));
        return Ok(());
    }
    task.step_with(
        "Located the firmware module folder",
        firmware_module_folder.display().to_string(),
    );

    let build_file_name = format!("{}module.rs", found_module.name);
    let is_module_file = firmware_module_folder.join(&build_file_name);

    if is_module_file.is_file() {
        task.Complete(format!(
            "'{}' already exists on disk, so it was left untouched",
            is_module_file.display()
        ));
        return Ok(());
    }

    let source_url = format!(
        "https://raw.githubusercontent.com/Adeun-Ilemobola/rust_esp32_based/refs/heads/{}/src/module/{}",
        BRANCH_NAME, &build_file_name
    );
    if let Err(error) = download_file(&source_url, &firmware_module_folder).await {
        task.fail(format!(
            "Could not download '{}' from {}: {}",
            build_file_name, source_url, error
        ));
        return Ok(());
    }
    task.step_with(
        format!("Downloaded '{}'", build_file_name),
        source_url,
    );

    if !update_config_file_with_component(&get_project_root, &found_module.name) {
        task.fail(format!(
            "Downloaded '{}' but could not record it in the project config",
            found_module.name
        ));
        return Ok(());
    }
    task.step_with(
        "Recorded the component in the project config",
        &found_module.name,
    );

    task.Complete(format!(
        "Installed '{}' into '{}'",
        found_module.name, data.project_name
    ));

    Ok(())
}
