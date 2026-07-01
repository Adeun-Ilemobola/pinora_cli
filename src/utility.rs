use crate::sharedTypes::{
    CargoDependency, LogType, ProgressEvent, ProgressLogShape, ProgressType, TAURI_DEPENDENCY_LIST,
    UI_TEMPLATE_LIST, UIReplacement, UISourceFile,
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

pub fn emit_progress(stage: &str, message: &str, current: u8, total: u8) {
    let event = ProgressEvent {
        stage,
        message,
        current,
        total,
    };

    let json = serde_json::to_string(&event).expect("Failed to serialize progress event");
    println!("__ESP_PROGRESS__:{}", json);
}

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

pub fn project_file_valid(current_dir: &std::path::Path) -> bool {
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

async fn  add_dependency(project_path: &str, dep: &CargoDependency, id: &str) -> std::io::Result<()> {
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

pub async fn pre_build_ui(project_path: &Path) -> bool {
    progress_log(
        ProgressType::Installing,
        "Installing Tauri app with bun...".to_string(),
        "pre-build-ui".to_string(),
    );

    let ui_project_name = "ui";

    let status = tokio::process::Command::new("bun")
        .args(["create", "tauri-app", ui_project_name, "-y"])
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

            let ui_project_path = project_path.join(ui_project_name);
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

            progress_log(
                ProgressType::Finished,
                "Tauri dependency installation complete.".to_string(),
                "pre-build-ui".to_string(),
            );

            if has_failed { false } else { true }
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
