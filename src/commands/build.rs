
use crate::sharedTypes::{LogType};
use crate::utility::{log  };
use crate::projectConfig::{load_config   };
use std::process::Command;

pub fn build_esp() -> bool {
    let get_config =
        load_config(&std::env::current_dir().expect("Failed to get current directory"));
    if get_config.is_none() {
        log(
            "Project config file does not exist in the current directory. Please provide a valid project path or create a project first.",
            "Project Validation",
            LogType::Error,
        );
        return false;
    }
    let config = get_config.unwrap();
    let build_script = &config.build_command;
    let status = Command::new("bash")
        .arg("-lc")
        .arg(build_script)
        .current_dir(&config.project_path)
        .status();
    match status {
        Ok(status) if status.success() => {
            log("Build completed successfully.", "Build", LogType::Complete);
            true
        }

        Ok(status) => {
            log(
                &format!("Build failed with exit status: {}", status),
                "Build",
                LogType::Error,
            );
            false
        }
        Err(e) => {
            log(
                &format!("Failed to run build command: {}", e),
                "Build",
                LogType::Error,
            );
            false
        }
    }
}
