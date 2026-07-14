use crate::progress::ProgressTask;
use crate::shared_types::{CargoDependency, LogType, NodeDependency};
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

/// Messages with no task behind them: CLI usage errors, and warnings that are worth showing but
/// are not a step and not a failure. Anything belonging to a task goes on a [`ProgressTask`].
/// Goes to stderr so stdout stays a clean protocol channel.
pub fn log(message: &str, milestone: &str, lt: LogType) {
    let label = match lt {
        LogType::Info => "INFO",
        LogType::Warning => "WARN",
        LogType::Error => "ERROR",
    };
    eprintln!("[{}] {}: {}", label, milestone, message);
}

pub fn get_available_serial_ports() -> anyhow::Result<Vec<String>> {
    let ports = serialport::available_ports()?;
    Ok(ports.into_iter().map(|port| port.port_name).collect())
}

/// Prompts on stderr, so the port list never lands in the middle of the protocol stream.
pub fn select_serial_port() -> Option<String> {
    let ports = match get_available_serial_ports() {
        Ok(ports) => ports,
        Err(err) => {
            log(&format!("Could not list serial ports: {}", err), "Serial Port", LogType::Error);
            return None;
        }
    };

    if ports.is_empty() {
        log(
            "No serial ports found. Plug in your ESP32 and try again.",
            "Serial Port",
            LogType::Error,
        );
        return None;
    }

    eprintln!("\nAvailable serial ports:\n");
    for (index, port) in ports.iter().enumerate() {
        eprintln!("[{}] {}", index + 1, port);
    }
    eprint!("\nSelect port number: ");
    io::stderr().flush().ok();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        log("Failed to read your selection.", "Serial Port", LogType::Error);
        return None;
    }

    let selected: usize = match input.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            log("Invalid selection: expected a number.", "Serial Port", LogType::Error);
            return None;
        }
    };

    if selected == 0 || selected > ports.len() {
        log("That port number is out of range.", "Serial Port", LogType::Error);
        return None;
    }

    Some(ports[selected - 1].clone())
}

pub fn add_dependency(
    project_path: &str,
    dep: &CargoDependency,
    task: &mut ProgressTask,
) -> io::Result<()> {
    let package = format!("{}@{}", dep.name, dep.version);
    task.step_with(
        format!("Adding crate '{}'", dep.name),
        format!("cargo add {}", package),
    );

    let mut command = Command::new("cargo");
    command.current_dir(project_path).arg("add").arg(&package);
    for feature in dep.features {
        command.arg("--features").arg(feature);
    }

    let status = command.status()?;
    if status.success() {
        return Ok(());
    }

    Err(io::Error::other(format!(
        "cargo add {} failed ({})",
        package, status
    )))
}

/// One `bun add` per batch, since dev and runtime packages need different flags.
pub fn add_node_dependencies(
    project_path: &Path,
    deps: &[NodeDependency],
    task: &mut ProgressTask,
) -> io::Result<()> {
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
        task.step_with(
            format!("Installing {} {} package(s) with bun", packages.len(), kind),
            packages.join(", "),
        );

        let mut command = Command::new("bun");
        command.current_dir(project_path).arg("add");
        if is_dev {
            command.arg("--dev");
        }
        command.args(&packages);

        let status = command.status()?;
        if !status.success() {
            return Err(io::Error::other(format!(
                "bun add failed for {} packages ({})",
                kind, status
            )));
        }
    }

    Ok(())
}

/// The number of steps [`add_node_dependencies`] will report, so the caller's total lines up.
pub fn node_dependency_steps(deps: &[NodeDependency]) -> u32 {
    let runtime = deps.iter().any(|dep| !dep.dev) as u32;
    let dev = deps.iter().any(|dep| dep.dev) as u32;
    runtime + dev
}

pub fn add_shadcn_components(
    project_path: &Path,
    components: &[&str],
    task: &mut ProgressTask,
) -> io::Result<()> {
    task.step_with(
        format!("Adding {} shadcn component(s)", components.len()),
        components.join(", "),
    );

    // The template's components.json is already in place, so the CLI skips init and honours
    // its style/baseColor. `bunx` resolves the shadcn version pinned in package.json.
    let mut command = Command::new("bunx");
    command
        .current_dir(project_path)
        .arg("--bun")
        .arg("shadcn")
        .arg("add")
        .args(components)
        .arg("--yes");

    let status = command.status()?;
    if status.success() {
        return Ok(());
    }

    Err(io::Error::other(format!("shadcn add failed ({})", status)))
}
