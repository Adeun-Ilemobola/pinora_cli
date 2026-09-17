mod commands;
mod component_core;
mod firmware;
mod global_definition;
mod module;
mod progress;
mod project_config;
mod project_config_database;
mod root;
mod ui;
mod utility;
use anyhow::Result;
use commands::build::build_esp;
use commands::create::pre_create;
use progress::ProgressTask;
use project_config::load_config;
use reqwest::header::{ACCEPT, USER_AGENT};
use std::env;
use std::path::Path;
use std::process::Command;
use utility::{log, select_serial_port};

use crate::{
    component_core::ComponentCore, firmware::firmware_definition::ESP_FOLDER_NAME, global_definition::LogType, project_config::update_config_file_with_component,
};





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
        log(
            "Please provide a command, e.g. `esp build`.",
            "Usage",
            LogType::Error,
        );
        log("Run `esp help` to see them all.", "Usage", LogType::Info);
        return;
    };

    match command.as_str() {
        "create" => match pre_create(&args).await {
            Ok(_) => {}
            Err(error) => {
                log(
                    &format!("Failed to create project: {:?}", error),
                    "Create",
                    LogType::Error,
                );
            }
        },

        "run" => {
            let _port = if args.len() >= 4 && args[2] == "--port" {
                Some(args[3].clone())
            } else {
                None
            };

            // build_esp and run_project_flash each report their own progress and reasons.
            if build_esp(true) {
                // run_project_flash(port);
            }
        }

        "build" => {
            build_esp(false);
        }

        "add" => {
            let mut task = ProgressTask::start("add <component>", 5, "Adding component");
            let Some(name) = args.get(2) else {
                log(
                    "Please provide a component name, e.g. `esp add ledmodule`.",
                    "Usage",
                    LogType::Error,
                );
                task.fail("Missing component name");
                return;
            };
            let mut component_creater = match ComponentCore::new() {
                Ok(com) => com,
                Err(error) => {
                    log(
                        &format!("Failed to initialize component creator: {:?}", error),
                        "Add Component",
                        LogType::Error,
                    );
                    task.fail("Failed to initialize component creator");
                    return;
                }
            };
            task.step("[Component] Initializing component creator");

            let name_cleaned = name
                .trim_end_matches(".rs")
                .to_uppercase()
                .trim()
                .to_string();
            component_creater
                .add_component(&name_cleaned, &mut task)
                .await;
            if update_config_file_with_component(&component_creater.root_dir, &name_cleaned) {
                task.step("[Component] Updated config file with component successfully");
            } else {
                task.fail("Failed to update config file with component");
            }
            task.finish(&format!("[Component] Added component '{}'", name_cleaned));
        }

        "listcomponents" => {
            let component_creater = match ComponentCore::new() {
                Ok(com) => com,
                Err(error) => {
                    log(
                        &format!("Failed to initialize component creator: {:?}", error),
                        "List Components",
                        LogType::Error,
                    );
                    return;
                }
            };
            let components = component_creater.list_all_components();
            if components.is_empty() {
                log("No components found.", "Components", LogType::Info);
            } else {
                println!("Available components ({}):", components.len());
                for (index, component) in components.iter().enumerate() {
                    println!("  {}. {}", index + 1, component);
                }
            }
        }

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
                &format!(
                    "Unknown command '{}'. Run `esp help` to see them all.",
                    unknown
                ),
                "Usage",
                LogType::Error,
            );
        }
    }
}
