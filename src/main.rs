mod commands;
mod file_json;
mod progress;
mod project_config;
mod project_config_database;
mod sharedtypes;
mod utility;

use anyhow::Result;
use commands::build::build_esp;
use commands::create::pre_create;
use progress::ProgressTask;
use project_config::{load_config, update_config_file_with_component};
use reqwest::header::{ACCEPT, USER_AGENT};
use sharedtypes::{BRANCH_NAME, GitHubItem, LogType};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use utility::{download_file, log, select_serial_port};

use crate::sharedtypes::ESP_FOLDER_NAME;

async fn load_all_modules() -> Result<Vec<GitHubItem>, String> {
    let url = &format!(
        "https://api.github.com/repos/Adeun-Ilemobola/rust_esp32_based/contents/src/module?ref={}",
        BRANCH_NAME
    );
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
    // load config, fetch registry, download, register
    let mut task = ProgressTask::start(
        "component",
        4,
        format!("Installing component '{}'", component_name),
    );

    let Ok(current_dir) = std::env::current_dir() else {
        task.fail("Could not read the current directory");
        return;
    };

    let Some(config) = load_config() else {
        task.fail(
            "No project config found here. Run `esp create <name>`, or cd into an existing project.",
        );
        return;
    };
    task.step_with("Loaded project config", &config.project_name);

    if config
        .install_components
        .iter()
        .any(|installed| installed.eq_ignore_ascii_case(component_name))
    {
        task.finish(format!("Component '{}' is already installed", component_name));
        return;
    }

    task.step("Fetching the component registry");
    let modules = match load_all_modules().await {
        Ok(modules) => modules,
        Err(error) => {
            task.fail(format!("Could not load the component registry: {}", error));
            return;
        }
    };

    let Some(component) = modules
        .into_iter()
        .find(|module| module.name.eq_ignore_ascii_case(component_name))
    else {
        task.fail(format!(
            "Component '{}' is not in the registry. Run `esp listcomponents` to see what is.",
            component_name
        ));
        return;
    };

    let module_folder = current_dir.join("src").join("module");
    let output_path = module_folder.join(&component.name);
    task.step_with("Downloading component", output_path.display().to_string());

    let source_url = format!(
        "https://raw.githubusercontent.com/Adeun-Ilemobola/rust_esp32_based/refs/heads/{}/src/module/{}",
        BRANCH_NAME, component.name
    );
    if let Err(error) = download_file(&source_url, &output_path).await {
        task.fail(format!(
            "Could not download component '{}': {}",
            component_name, error
        ));
        return;
    }

    task.step_with("Registering component", "src/module/mod.rs");
    let mut all_components = config.install_components.clone();
    all_components.push(component_name.to_string());

    let mod_contents = all_components
        .iter()
        .map(|name| name.trim().replace(".rs", "").to_lowercase())
        .map(|name| format!("pub mod {};", name))
        .collect::<Vec<String>>()
        .join("\n");

    if let Err(error) = fs::write(module_folder.join("mod.rs"), mod_contents) {
        task.fail(format!("Could not update src/module/mod.rs: {}", error));
        return;
    }

    if !update_config_file_with_component(&current_dir, component_name) {
        task.fail(format!(
            "Downloaded '{}' but could not record it in the project config",
            component_name
        ));
        return;
    }

    task.finish(format!(
        "Installed '{}' at {}",
        component_name,
        output_path.display()
    ));
}

fn run_project_flash(port: Option<String>) -> bool {
    // load config, choose port, locate binary, flash
    let mut task = ProgressTask::start("flash", 4, "Preparing to flash");

    let Some(config) = load_config() else {
        task.fail(
            "No project config found here. Run `esp create <name>`, or cd into an existing project.",
        );
        return false;
    };
    task.step_with("Loaded project config", &config.project_name);

    // An explicit --port is used as given; only prompt when one was not supplied.
    let selected_port = match port {
        Some(port) if !port.trim().is_empty() => port,
        _ => match select_serial_port() {
            Some(port) => port,
            None => {
                task.fail("No serial port selected, so nothing was flashed");
                return false;
            }
        },
    };
    task.step_with("Selected serial port", &selected_port);

    let elf_path = Path::new(&config.firmware_path)
        .join("target")
        .join("xtensa-esp32-espidf")
        .join("debug")
        .join(ESP_FOLDER_NAME);

    if !elf_path.exists() {
        task.fail(format!(
            "No firmware binary at {}. Run `esp build` first.",
            elf_path.display()
        ));
        return false;
    }
    task.step_with("Located firmware binary", elf_path.display().to_string());

    task.step_with(
        format!("Flashing '{}' to {}", config.project_name, selected_port),
        "espflash flash --monitor",
    );

    // Inherits stdio, so the device monitor stays interactive.
    let status = Command::new("espflash")
        .arg("flash")
        .arg("--monitor")
        .arg(&elf_path)
        .env("ESPFLASH_PORT", &selected_port)
        .current_dir(&config.firmware_path)
        .status();

    match status {
        Ok(status) if status.success() => {
            task.finish(format!(
                "Flashed '{}' to {}",
                config.project_name, selected_port
            ));
            true
        }
        Ok(status) => {
            task.fail(format!("Flashing failed ({})", status));
            false
        }
        Err(error) => {
            task.fail(format!("Could not run espflash: {}", error));
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
     ["project", "listcomponents"]
    */
    let Some(command) = args.get(1) else {
        log("Please provide a command, e.g. `esp build`.", "Usage", LogType::Error);
        log("Run `esp help` to see them all.", "Usage", LogType::Info);
        return;
    };

    match command.as_str() {
        "create" => {
            pre_create(&args).await;
        }

        "run" => {
            let port = if args.len() >= 4 && args[2] == "--port" {
                Some(args[3].clone())
            } else {
                None
            };

            // build_esp and run_project_flash each report their own progress and reasons.
            if build_esp() {
                run_project_flash(port);
            }
        }

        "build" => {
            build_esp();
        }

        "add" => {
            let Some(name) = args.get(2) else {
                log(
                    "Please provide a component name, e.g. `esp add ledmodule`.",
                    "Usage",
                    LogType::Error,
                );
                return;
            };
            let component_name = if name.ends_with(".rs") {
                name.clone()
            } else {
                format!("{}.rs", name)
            };
            install_component(&component_name).await;
        }

        "listcomponents" => match load_all_modules().await {
            Ok(modules) if modules.is_empty() => {
                log("No components found in the registry.", "Components", LogType::Info);
            }
            Ok(modules) => {
                println!("Available components ({}):", modules.len());
                for (index, module) in modules.iter().enumerate() {
                    println!("  {}. {}", index + 1, module.name.trim_end_matches(".rs"));
                }
            }
            Err(error) => {
                log(
                    &format!("Could not fetch components: {}", error),
                    "Components",
                    LogType::Error,
                );
            }
        },

        "help" => {
            println!("Available commands:");
            println!("  create <name> [--path <dir>]  Create a new project.");
            println!("  build                         Build the firmware.");
            println!("  run [--port <port>]           Build, then flash to the device.");
            println!("  add <component>               Install a component.");
            println!("  listcomponents                List available components.");
            println!("  help                          Show this message.");
        }

        unknown => {
            log(
                &format!("Unknown command '{}'. Run `esp help` to see them all.", unknown),
                "Usage",
                LogType::Error,
            );
        }
    }
}
