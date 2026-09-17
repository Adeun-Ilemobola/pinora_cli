use std::path::{Path, PathBuf};

use crate::{firmware::firmware_definition::ESP_FOLDER_NAME, global_definition::ProjectConfig, project_config_database::update_project_config, ui::ui_definition::UI_FOLDER_NAME};

const CONFIG_RELATIVE_PATH: &str = ".pinora/project_config.json";

/// Looks for the project config in the current directory, then in the firmware and UI
/// subdirectories, so commands work from anywhere inside a project.
pub fn load_config() -> Option<ProjectConfig> {
    let root_dir = std::env::current_dir().ok()?;

    let candidates: [PathBuf; 3] = [
        root_dir.clone(),
        root_dir.join(ESP_FOLDER_NAME),
        root_dir.join(UI_FOLDER_NAME),
    ];

    for candidate in candidates {
        let config_path = candidate.join(CONFIG_RELATIVE_PATH);
        if !config_path.exists() {
            continue;
        }
        let contents = std::fs::read_to_string(&config_path).ok()?;
        return serde_json::from_str(&contents).ok();
    }

    None
}

pub fn save_config(path: &Path, config: &ProjectConfig) -> Option<ProjectConfig> {
    let config_path = path.join(CONFIG_RELATIVE_PATH);

    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).ok()?;
    }
    let serialised = serde_json::to_string_pretty(config).ok()?;
    std::fs::write(&config_path, serialised).ok()?;

    Some(config.clone())
}

pub fn update_config_file_with_component(project_path: &Path, component_name: &str) -> bool {
    let Some(mut config) = load_config() else {
        return false;
    };

    config.install_components.push(component_name.to_string());

    if save_config(project_path, &config).is_none() {
        return false;
    }

    update_project_config(&config);
    true
}

/// Returns why the name is unusable, or `None` if it is fine.
pub fn project_name_error(project_name: &str) -> Option<&'static str> {
    if project_name.trim().is_empty() {
        return Some("it cannot be empty");
    }
    if project_name.contains(' ') {
        return Some("it cannot contain spaces");
    }
    if project_name.contains('/') || project_name.contains('\\') {
        return Some("it cannot contain path separators");
    }
    if project_name.contains('.') {
        return Some("it cannot contain dots");
    }
    if project_name.contains('-') {
        return Some("it cannot contain hyphens");
    }
    if project_name.len() < 3 {
        return Some("it must be at least 3 characters");
    }
    if project_name.len() > 100 {
        return Some("it must be at most 100 characters");
    }
    None
}

impl ProjectConfig {
    /// Derive build output from the existing identity, never from a source path or name.
    /// The platform is explicit so both scaffold policies can be tested on any host.
    pub(crate) fn firmware_target_dir(&self, windows: bool) -> Result<String, uuid::Error> {
        if !windows {
            return Ok("target".to_owned());
        }
        let id = uuid::Uuid::parse_str(&self.id)?;
        // Fixed FNV-1a over all 16 bytes; do not use DefaultHasher, whose algorithm
        // is not stable across Rust versions. The key must survive CLI upgrades.
        let hash = id.as_bytes().iter().fold(0xcbf29ce484222325u64, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
        });
        let mut number = hash % 36u64.pow(5);
        let mut key = [b'0'; 5];
        for digit in key.iter_mut().rev() {
            *digit = b"0123456789abcdefghijklmnopqrstuvwxyz"[(number % 36) as usize];
            number /= 36;
        }
        // Exactly 10 ASCII characters, including drive, separators and key.
        Ok(format!("C:/p/{}", std::str::from_utf8(&key).unwrap()))
    }
}

/// Reserve a compact key for its full UUID before publishing the Cargo config.
/// A sibling file survives Cargo clean. Atomic creation prevents two scaffolders
/// from silently assigning the same key to different projects.
pub(crate) fn claim_firmware_target(target: &Path, owner: uuid::Uuid) -> std::io::Result<()> {
    use std::io::{Error, ErrorKind, Write};
    let marker = target.with_extension("owner");
    let verify_owner = || -> std::io::Result<()> {
        let saved = std::fs::read_to_string(&marker)?;
        if uuid::Uuid::parse_str(saved.trim()).ok() == Some(owner) {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::AlreadyExists, format!(
                "Firmware target {} is reserved by another UUID or has an invalid owner file. Refusing to share build artifacts; create a project with a new UUID.",
                target.display()
            )))
        }
    };
    match verify_owner() {
        Ok(()) => return Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    if target.try_exists()? {
        return Err(Error::new(ErrorKind::AlreadyExists, format!(
            "Firmware target {} already exists without an owner file; refusing to reuse it.",
            target.display()
        )));
    }
    let parent = target.parent().ok_or_else(|| Error::new(ErrorKind::InvalidInput, "Missing target parent"))?;
    std::fs::create_dir_all(parent)?;
    match std::fs::OpenOptions::new().write(true).create_new(true).open(&marker) {
        Ok(mut file) => {
            // Interrupted/incomplete writes remain reserved and fail closed.
            writeln!(file, "{owner}")?;
            file.sync_all()
        }
        Err(error) if error.kind() == ErrorKind::AlreadyExists => verify_owner(),
        Err(error) => Err(error),
    }
}
