mod sharedtypes;
mod utility;
mod commands;
mod file_json;
mod project_config_database;
mod project_config;
use std::env;
use std::fs;
use std::path::Path;
use sharedtypes::{GitHubItem, LogType , BRANCH_NAME};
use utility::{download_file  , log  ,select_serial_port  };
use project_config::{load_config   , update_config_file_with_component };
use anyhow::{ Result};
use reqwest::header::{ACCEPT, USER_AGENT};
use commands::create::{pre_create,};
use commands::build::{build_esp,};

use std::process::Command;
use tokio;

use crate::sharedtypes::ESP_FOLDER_NAME;



async fn load_all_modules() -> Result<Vec<GitHubItem>, String> {
    let url = &format!("https://api.github.com/repos/Adeun-Ilemobola/rust_esp32_based/contents/src/module?ref={}", BRANCH_NAME);
    let client = reqwest::Client::new();
    let response: Vec<GitHubItem> = client
        .get(url)
        .header(USER_AGENT, "Mozilla/5.0 (compatible; MyRustApp/1.0)")
        .header(ACCEPT, "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|err| err.to_string())?
        .json::<Vec<GitHubItem>>()
        .await
        .map_err(|err| err.to_string())?;

    let modules: Vec<GitHubItem> = response
        .into_iter()
        .filter(|item| {
            item.item_type == "file"
                && item.name.ends_with(".rs")
                && item.name != "mod.rs"
                && item.name != "ModuleCore.rs"
        })
        
        .collect();

    Ok(modules)
}

async fn install_component(component_name: &str) {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    let get_config = load_config();
    if get_config.is_none() {
        log(
            "Project config file does not exist in the current directory. Please provide a valid project path or create a project first.",
            "Component Installation",
            LogType::Error,
        );
        return;
    }

    let modules = load_all_modules().await;

    match modules {
        Ok(modules) => {
            if let Some(component) = modules
                .into_iter()
                .find(|m| m.name.to_lowercase() == component_name.to_lowercase())
            {
                if get_config
                    .as_ref()
                    .unwrap()
                    .install_components
                    .iter()
                    .any(|c| c.to_lowercase() == component_name.to_lowercase())
                {
                    log(
                        &format!(
                            "Component '{}' is already installed in the project.",
                            component_name
                        ),
                        "Component Installation",
                        LogType::Warning,
                    );
                    return;
                }


                log(
                    &format!("Component '{}' found in registry.", component_name),
                    "Component Installation",
                    LogType::Info,
                );

                let new_component = &format!(
                    "https://raw.githubusercontent.com/Adeun-Ilemobola/rust_esp32_based/refs/heads/{}/src/module/{}",
                    BRANCH_NAME,
                    component.name
                );
                let module_folder_path = current_dir.join("src").join("module");
                let output_path = module_folder_path.join(&component.name);

                match download_file(new_component, &output_path).await {
                    Ok(_) => {
                        log(
                            &format!(
                                "Component '{}' installed successfully at {}.",
                                component_name,
                                output_path.display()
                            ),
                            "Component Installation",
                            LogType::Info,
                        );

                        let mod_file = module_folder_path.join("mod.rs");
                        let mut all_components = get_config.unwrap().install_components.clone();
                        all_components.push(component_name.to_string());

                        let mod_contents = all_components
                            .iter()
                            .map(|text| text.trim().replace(".rs", ""))
                            .map(|c| format!("pub mod {};", c.to_lowercase()))
                            .collect::<Vec<String>>()
                            .join("\n");

                        fs::write(&mod_file, mod_contents).expect("Failed to write to mod.rs");

                        let update_success =
                            update_config_file_with_component(&current_dir, component_name);
                        if !update_success {
                            log(
                                &format!(
                                    "Failed to update project config with component '{}'.",
                                    component_name
                                ),
                                "Component Installation",
                                LogType::Error,
                            );
                        }
                    }
                    Err(e) => log(
                        &format!("Failed to install component '{}': {}", component_name, e),
                        "Component Installation",
                        LogType::Error,
                    ),
                }
            } else {
                log(
                    &format!("Component '{}' not found in registry.", component_name),
                    "Component Installation",
                    LogType::Error,
                );
            }
        }
        Err(e) => {
            log(
                &format!("Failed to load module registry: {}", e),
                "Component Installation",
                LogType::Error,
            );
        }
    }
}


fn run_project_flash(port: Option<String>) -> bool {
    let get_config =load_config();

    if get_config.is_none() {
        log(
            "Project config file does not exist in the current directory. Please provide a valid project path or create a project first.",
            "Project Validation",
            LogType::Error,
        );
        return false;
    }
    let config = get_config.unwrap();

    let get_valid_port = select_serial_port(port.is_some());

    if get_valid_port.is_none() {
        log(
            "No valid serial port selected. Flashing process aborted.",
            "Serial Port Selection",
            LogType::Error,
        );
        return false;
    }
    let mut selected_port = get_valid_port.unwrap();

    if port.is_some() {
        selected_port = port.unwrap();
    }

    if selected_port.is_empty() {
        log(
            "Selected serial port is empty. Flashing process aborted.",
            "Serial Port Selection",
            LogType::Error,
        );
        return false;
    }

    let elf_path = Path::new(&config.firmware_path)
        .join("target")
        .join("xtensa-esp32-espidf")
        .join("debug")
        .join(&ESP_FOLDER_NAME);

    if !elf_path.exists() {
        log(
            &format!("Built ELF file not found at: {}", elf_path.display()),
            "Flashing",
            LogType::Error,
        );
        return false;
    }

    let status = Command::new("espflash")
        .arg("flash")
        .arg("--monitor")
        .arg(&elf_path)
        .env("ESPFLASH_PORT", &selected_port)
        .current_dir(&config.firmware_path)
        .status();

    match status {
        Ok(status) if status.success() => {
            log(
                "Flashing completed successfully.",
                "Flashing",
                LogType::Complete,
            );

            true
        }
        Ok(status) => {
            log(
                &format!("Flashing failed with exit status: {}", status),
                "Flashing",
                LogType::Error,
            );
            false
        }
        Err(e) => {
            log(
                &format!("Failed to run flash command: {}", e),
                "Flashing",
                LogType::Error,
            );
            false
        }
    }
}

#[tokio::main]
async fn main() {
    

    let args: Vec<String> = env::args().collect();
    /*
     ["project", "create" , "project_name" , '--path' , 'path/to/project']
     ["project", "run" , "--port" , "serial_port"]
     ["project" , "build" ]
     ["project", "help"]
     ["project", "add" , "{component_name}"]
     ["project", "remove" , "{component_name}"]
     ["project", "update" , "{component_name}"]
     ["project", "listcomponents"]

    */
    if args.len() < 2 {
        log(
            "Please provide a command.",
            "Command Validation",
            LogType::Error,
        );
        log("Example: project run", "Command Validation", LogType::Info);
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "create" => {
            pre_create(&args).await;
        }
        "run" => {
            if args.len() >= 4 && args[2] == "--port" {
                let port = args[3].clone();
                let valid_build = build_esp();
                if !valid_build {
                    log("Build failed. Cannot flash project.", "Run", LogType::Error);
                    return;
                }
                let flash_result = run_project_flash(Some(port));
                if flash_result {
                    log("Project flashed successfully!", "Run", LogType::Complete);
                } else {
                    log("Failed to flash project.", "Run", LogType::Error);
                }
            } else {
                let valid_build = build_esp();
                if !valid_build {
                    log("Build failed. Cannot flash project.", "Run", LogType::Error);
                    return;
                }
                let flash_result = run_project_flash(None);
                if flash_result {
                    log("Project flashed successfully!", "Run", LogType::Complete);
                } else {
                    log("Failed to flash project.", "Run", LogType::Error);
                }
            }
        }

        "add" => {
            if args.len() < 3 {
                log(
                    "Please provide a component name to install.",
                    "Command Validation",
                    LogType::Error,
                );
                return;
            }
            let mut component_name = args[2].clone();
            if !component_name.ends_with(".rs") {
                component_name = format!("{}.rs", &component_name);
            }
            install_component(&component_name).await;
        }

        "build" => {
            build_esp();
        }

        "listcomponents" => {
            match load_all_modules().await {
                Ok(modules) => {
                    if modules.is_empty() {
                        log("No components found in registry.", "List Components", LogType::Info);
                    } else {
                        println!("Available components ({}):", modules.len());
                        for (i, module) in modules.iter().enumerate() {
                            let display_name = module.name.trim_end_matches(".rs");
                            println!("  {}. {}", i + 1, display_name);
                        }
                    }
                }
                Err(e) => {
                    log(
                        &format!("Failed to fetch components: {}", e),
                        "List Components",
                        LogType::Error,
                    );
                }
            }
        }

        "help" => {
            println!("Available commands:");
            println!("create <project_name> - Create a new project with the specified name.");
            println!("run - Flash the project to the ESP device.");
            println!("build - Build the project.");
            println!("help - Show this help message.");
            println!("install <component_name> - Install a component.");
            println!("uninstall <component_name> - Uninstall a component.");
            println!("update <component_name> - Update a component.");
            println!("listcomponents - List all available components.");
        }

        _ => {
            println!("Unknown command: {}", command);
            println!("Available commands: create, run, build, help");
        }
    }
}
