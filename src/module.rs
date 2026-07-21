use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tokio;

use crate::progress::ProgressTask;
use crate::project_config::{load_config, update_config_file_with_component};
// use crate::shared_types::BRANCH_NAME;
use crate::utility::download_file;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ModuleShape {
    pub name: String,
    pub source: String,
    pub default: bool,
}

fn update_module_mod_file(module_folder: &Path, module_name: &str) -> Result<bool> {
    let mod_file = module_folder.join("mod.rs");
    let mut contents = fs::read_to_string(&mod_file)
        .with_context(|| format!("Failed to read {}", mod_file.display()))?;
    let declaration = format!("pub mod {}module;", module_name);

    if contents.lines().any(|line| line.trim() == declaration) {
        return Ok(false);
    }

    if !contents.is_empty() && !contents.ends_with('\n') {
        contents.push('\n');
    }
    contents.push_str(&declaration);
    contents.push('\n');

    fs::write(&mod_file, contents)
        .with_context(|| format!("Failed to write {}", mod_file.display()))?;

    Ok(true)
}

pub async fn load_modules() -> Result<Vec<ModuleShape>, anyhow::Error> {
    println!("{:?}", std::env::current_dir()?);
    let modules_data_file = PathBuf::from("/Users/adeun/Pinora_Project/Pinora_CLI/src/moduleDatabase.json");

    let json_text = tokio::fs::read_to_string(&modules_data_file)
        .await
        .context("Failed to read JSON file")?;

    let files: Vec<ModuleShape> =
        serde_json::from_str(&json_text).context("Failed to parse JSON file")?;

    Ok(files)
}

pub async fn add_modules(name: String) -> Result<(), anyhow::Error> {
    let mut task = ProgressTask::start("add module", 9, format!("Installing component '{}'", name));

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

    let already_recorded = data.install_components.contains(&found_module.name);
    if already_recorded {
        task.step("Component was already recorded; checking its files");
    } else {
        task.step("Confirmed the component is not installed yet");
    }

    let firmware_path = PathBuf::from(&data.firmware_path);

    let Some(get_project_root) = firmware_path.parent().map(Path::to_path_buf) else {
        task.fail(format!(
            "Firmware path '{}' has no parent directory, so the project root cannot be resolved",
            firmware_path.display()
        ));
        return Ok(());
    };
    task.step_with(
        "Resolved project root",
        get_project_root.display().to_string(),
    );

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
        task.step_with(
            "Component source file already exists",
            is_module_file.display().to_string(),
        );
    } else {
       
        if let Err(error) = download_file(&found_module.source, &is_module_file).await {
            task.fail(format!(
                "Could not download '{}' from {}: {}",
                build_file_name, &found_module.source, error
            ));
            return Ok(());
        }
        task.step_with(format!("Downloaded '{}'", build_file_name), &found_module.source);
    }

    match update_module_mod_file(&firmware_module_folder, &found_module.name) {
        Ok(true) => task.step_with(
            "Updated firmware module registry",
            format!("pub mod {}module;", found_module.name),
        ),
        Ok(false) => task.step("Firmware module registry was already up to date"),
        Err(error) => {
            task.fail(format!(
                "Downloaded '{}' but could not update src/module/mod.rs: {}",
                found_module.name, error
            ));
            return Ok(());
        }
    }

    if already_recorded {
        task.step("Project config was already up to date");
    } else {
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
    }

    task.Complete(format!(
        "Installed '{}' into '{}'",
        found_module.name, data.project_name
    ));

    Ok(())
}
