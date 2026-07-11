use crate::sharedtypes::{
    CargoDependency, LogType, NodeDependency, ProgressLogShape, ProgressType,
    
};
use anyhow::Result;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub async fn download_file(git_url: &str, output_path: &Path) -> Result<()> {
    let content = reqwest::get(git_url).await?.text().await?;
    if let Some(parent) = output_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(output_path, content).await?;
    Ok(())
}

// pub fn emit_progress(stage: &str, message: &str, current: u8, total: u8) {
//     let event = ProgressEvent {
//         stage,
//         message,
//         current,
//         total,
//     };

//     let json = serde_json::to_string(&event).expect("Failed to serialize progress event");
//     println!("__ESP_PROGRESS__:{}", json);
// }

pub fn log(message: &str, milestone: &str, lt: LogType) {
    println!("\n");
    let text_for_log = format!(
        "[{}] - {}: {}",
        match lt {
            LogType::Info => "INFO",
            LogType::Warning => "WARNING",
            LogType::Error => "ERROR",
            LogType::Complete => "COMPLETE",
        },
        milestone,
        message
    );
    println!("{}", text_for_log);
}

pub fn _project_file_valid(current_dir: &std::path::Path) -> bool {
    if !current_dir.join(".espConfig/esp_config.json").exists() {
        log(
            "Project config file does not exist.",
            "Project Validation",
            LogType::Error,
        );
        return false;
    }
    true
}

pub fn get_available_serial_ports() -> anyhow::Result<Vec<String>> {
    let ports = serialport::available_ports()?;
    Ok(ports.into_iter().map(|port| port.port_name).collect())
}

pub fn select_serial_port(is_manual: bool) -> Option<String> {
    let ports = get_available_serial_ports();
    match ports {
        Ok(listport) => {
            if is_manual {
                return None;
            }

            if listport.is_empty() {
                log(
                    "No serial ports found. Plug in your ESP32 and try again.",
                    "Serial Port Detection",
                    LogType::Error,
                );
                return None;
            }

            println!("\nAvailable serial ports:\n");

            for (index, port) in listport.iter().enumerate() {
                println!("[{}] {}", index + 1, port);
            }

            print!("\nSelect port number: ");
            io::stdout().flush().ok();

            let mut input = String::new();

            if io::stdin().read_line(&mut input).is_err() {
                log(
                    "Failed to read selected port.",
                    "Serial Port Selection",
                    LogType::Error,
                );
                return None;
            }

            let selected_number: usize = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    log(
                        "Invalid selection. Please enter a number.",
                        "Serial Port Selection",
                        LogType::Error,
                    );
                    return None;
                }
            };

            if selected_number == 0 || selected_number > listport.len() {
                log(
                    "Selected port number is out of range.",
                    "Serial Port Selection",
                    LogType::Error,
                );
                return None;
            }

            Some(listport[selected_number - 1].clone())
        }

        Err(err) => {
            log(
                &format!("port Errr:{}", err.to_string()),
                "port",
                LogType::Error,
            );
            return None;
        }
    }
}

pub fn progress_log(stage: ProgressType, message: String, id: String) {
    let event = ProgressLogShape { stage, id, message };
    let json = serde_json::to_string(&event).expect("Failed to serialize progress event");
    println!("__ESP_PROGRESS__:{}", json);
}

pub async fn  add_dependency(project_path: &str, dep: &CargoDependency, id: &str) -> std::io::Result<()> {
    progress_log(
        ProgressType::Installing,
        format!("Adding dependency '{}'...", dep.name),
        id.to_string(),
    );

    let package = format!("{}@{}", dep.name, dep.version);

    let mut command = Command::new("cargo");

    command.current_dir(project_path).arg("add").arg(package);

    for feature in dep.features {
        command.arg("--features").arg(feature);
    }

    let status = command.status()?;
    if status.success() {
        progress_log(
            ProgressType::Complete,
            format!("Dependency '{}' added.", dep.name),
            id.to_string(),
        );
        Ok(())
    } else {
        progress_log(
            ProgressType::Error,
            format!("Failed to add dependency '{}'.", dep.name),
            id.to_string(),
        );
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "cargo add failed for '{}' (exit status: {})",
                dep.name, status
            ),
        ))
    }
}

pub async fn add_node_dependencies(
    project_path: &Path,
    deps: &[NodeDependency],
    id: &str,
) -> std::io::Result<()> {
    for is_dev in [false, true] {
        let packages: Vec<String> = deps
            .iter()
            .filter(|dep| dep.dev == is_dev)
            .map(|dep| format!("{}@{}", dep.name, dep.version))
            .collect();

        if packages.is_empty() {
            continue;
        }

        let kind = if is_dev { "dev" } else { "runtime" };

        progress_log(
            ProgressType::Installing,
            format!("Installing {} {} package(s) with bun...", packages.len(), kind),
            id.to_string(),
        );

        let mut command = Command::new("bun");
        command.current_dir(project_path).arg("add");
        if is_dev {
            command.arg("--dev");
        }
        command.args(&packages);

        let status = command.status()?;
        if !status.success() {
            progress_log(
                ProgressType::Error,
                format!("Failed to install {} packages.", kind),
                id.to_string(),
            );
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("bun add failed for {} packages (exit status: {})", kind, status),
            ));
        }

        progress_log(
            ProgressType::Complete,
            format!("Installed {} {} package(s).", packages.len(), kind),
            id.to_string(),
        );
    }

    Ok(())
}

pub async fn add_shadcn_components(
    project_path: &Path,
    components: &[&str],
    id: &str,
) -> std::io::Result<()> {
    progress_log(
        ProgressType::Installing,
        format!("Adding {} shadcn component(s)...", components.len()),
        id.to_string(),
    );

    // The template's components.json is already in place, so the CLI skips init and honours
    // its style/baseColor. `bunx` resolves the shadcn version pinned in package.json.
    let status = Command::new("bunx")
        .current_dir(project_path)
        .arg("--bun")
        .arg("shadcn")
        .arg("add")
        .args(components)
        .arg("--yes")
        // .arg("--overwrite")
        .status()?;

    if status.success() {
        progress_log(
            ProgressType::Complete,
            format!("Added {} shadcn component(s).", components.len()),
            id.to_string(),
        );
        Ok(())
    } else {
        progress_log(
            ProgressType::Error,
            "Failed to add shadcn components.".to_string(),
            id.to_string(),
        );
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("shadcn add failed (exit status: {})", status),
        ))
    }
}

