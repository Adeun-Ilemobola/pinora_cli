use crate::progress::ProgressTask;
use crate::project_config::load_config;
use std::process::Command;

pub fn build_esp() -> bool {
    let mut task = ProgressTask::start("build", 2, "Preparing build");

    let Some(config) = load_config() else {
        task.fail(
            "No project config found here. Run `esp create <name>`, or cd into an existing project.",
        );
        return false;
    };
    task.step_with("Loaded project config", &config.project_name);

    task.step_with(
        format!("Building '{}'", config.project_name),
        &config.build_command,
    );

    // Inherits stdio, so cargo's own output goes straight to the terminal.
    let status = Command::new("bash")
        .arg("-lc")
        .arg(&config.build_command)
        .current_dir(&config.firmware_path)
        .status();

    match status {
        Ok(status) if status.success() => {
            task.finish(format!("Build succeeded for '{}'", config.project_name));
            true
        }
        Ok(status) => {
            task.fail(format!(
                "Build failed for '{}' ({})",
                config.project_name, status
            ));
            false
        }
        Err(error) => {
            task.fail(format!("Could not start the build command: {}", error));
            false
        }
    }
}
