use crate::global_definition::{
    LogType, ProjectConfig, SourceTemplate, TemplateEdit, TemplateValue,
};

use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Write};
use std::path::{Path};
#[derive(Debug, Clone, Copy)]
enum FileAction<'a> {
    Replace{file: &'a Path, new_content: &'a str},
    Insert{file: &'a Path, target: &'a str, new_content: &'a str, is_new_line: bool},
}
#[derive(Debug)]
pub enum FileActionError {
    InvalidTemplate(String),
    FileDoesNotExist(String),
    FailedToReadFile(String),
    FailedToWriteFile(String),
}
impl std::fmt::Display for FileActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileActionError::InvalidTemplate(message) => write!(f, "Invalid template: {}", message),
            FileActionError::FileDoesNotExist(file) => write!(f, "File does not exist: {}", file),
            FileActionError::FailedToReadFile(file) => write!(f, "Failed to read file: {}", file),
            FileActionError::FailedToWriteFile(file) => write!(f, "Failed to write file: {}", file),
        }
    }
}


pub async fn download_file(git_url: &str, output_path: &Path) -> Result<()> {
    let content = reqwest::get(git_url).await?.error_for_status()?.bytes().await?;
    if let Some(parent) = output_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(output_path, content).await?;
    Ok(())
}


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
            log(
                &format!("Could not list serial ports: {}", err),
                "Serial Port",
                LogType::Error,
            );
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
        log(
            "Failed to read your selection.",
            "Serial Port",
            LogType::Error,
        );
        return None;
    }

    let selected: usize = match input.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            log(
                "Invalid selection: expected a number.",
                "Serial Port",
                LogType::Error,
            );
            return None;
        }
    };

    if selected == 0 || selected > ports.len() {
        log(
            "That port number is out of range.",
            "Serial Port",
            LogType::Error,
        );
        return None;
    }

    Some(ports[selected - 1].clone())
}


pub fn file_change(file: &Path, target: &str, new_content: &str, is_new_line: bool) -> Result<(), FileActionError> {
    if !file.is_file() {
        return Err(FileActionError::FileDoesNotExist(file.display().to_string()));
    }

    let content =
        fs::read_to_string(&file).map_err(|_| FileActionError::FailedToReadFile(file.display().to_string()))?;
    if is_new_line {
        let target_position = content
            .find(target)
            .ok_or(FileActionError::FailedToReadFile(file.display().to_string()))?;
        let insertion_position = target_position + target.len();
        let (before, after) = content.split_at(insertion_position);

        let updated_content = format!("{before}\n{new_content}{after}");

        fs::write(&file, updated_content)
            .map_err(|_| FileActionError::FailedToWriteFile(file.display().to_string()))?;

        return Ok(());
    }
    let data = content.replacen(target, &new_content, 1);
    fs::write(&file, data).map_err(|_| FileActionError::FailedToWriteFile(file.display().to_string()))?;

    Ok(())
}

pub fn file_replace(file: &Path, new_content: &str) -> Result<(), FileActionError> {
    fs::write(&file, "").map_err(|_| FileActionError::FailedToWriteFile(file.display().to_string()))?;

    fs::write(&file, new_content).map_err(|_| FileActionError::FailedToWriteFile(file.display().to_string()))?;

    Ok(())
}


 fn file_action<'a>(action: FileAction<'a>) -> Result<(), FileActionError> {
    match action {
        FileAction::Insert { file, target, new_content, is_new_line } => {
            file_change(file, target, new_content, is_new_line)
        }
        FileAction::Replace { file, new_content } => {
            file_replace(file, new_content)
        }
    }
}


pub async fn generate_file(
    source: &SourceTemplate,
    root_dir: &Path,
    config: &ProjectConfig,
) -> Result<(), FileActionError> {

    let output_path = root_dir.join(source.output_path);

    if output_path.exists() {
        return Ok(());
    }

    // Reserve before downloading/writing a usable Cargo config. A five-digit key
    // can collide, so an existing different owner must stop scaffolding.
    if cfg!(windows) && source.edits.iter().any(|edit| matches!(
        edit, TemplateEdit::SetTomlString { value: TemplateValue::FirmwareTargetDir, .. }
    )) {
        let target = resolve_template_value(&TemplateValue::FirmwareTargetDir, config, true)?;
        let owner = uuid::Uuid::parse_str(&config.id)
            .map_err(|error| FileActionError::InvalidTemplate(error.to_string()))?;
        crate::project_config::claim_firmware_target(Path::new(&target), owner)
            .map_err(|error| FileActionError::FailedToWriteFile(error.to_string()))?;
    }
    if let Some(parent) = output_path.parent() {
        match fs::create_dir_all(parent) {
            Ok(_) => {}
            Err(err) => {
                return Err(FileActionError::FailedToWriteFile(format!("At create_dir_all :{:?}", err)));
            }
        }
    }

    if let Err(error) = download_file(source.github_source_url, output_path.as_path()).await {
        log(
            &format!(
                "Could not download root template {}: {:?}",
                source.name, error
            ),
            "Create",
            LogType::Error,
        );
        return Err(FileActionError::FailedToWriteFile(format!(
            "Could not download root template {}: {:?}",
            source.name, error
        )));
    }

    for edit in source.edits {
        apply_edit(output_path.as_path(), edit, config)?;
    }

    Ok(())
}

fn resolve_template_value(
    value: &TemplateValue,
    config: &ProjectConfig,
    windows: bool,
) -> Result<String, FileActionError> {
    Ok(match value {
        TemplateValue::Literal(info) => info.to_string(),
        TemplateValue::FirmwarePath => config.firmware_path.clone(),
        TemplateValue::UiPath => config.ui_path.clone(),
        TemplateValue::ProjectName => config.project_name.clone(),
        TemplateValue::FirmwareTargetDir => config
            .firmware_target_dir(windows)
            .map_err(|error| FileActionError::InvalidTemplate(format!("Invalid project UUID: {error}")))?,
    })
}

fn apply_edit(output_path: &Path, item: &TemplateEdit, config: &ProjectConfig) -> Result<(), FileActionError> {
    apply_edit_for_platform(output_path, item, config, cfg!(windows))
}

fn apply_edit_for_platform(
    output_path: &Path,
    item: &TemplateEdit,
    config: &ProjectConfig,
    windows: bool,
) -> Result<(), FileActionError> {
    match item {
        TemplateEdit::SetTomlString { table, key, value } => {
            let data = resolve_template_value(value, config, windows)?;
            let text = fs::read_to_string(output_path)
                .map_err(|_| FileActionError::FailedToReadFile(output_path.display().to_string()))?;
            let mut document = text.parse::<toml_edit::DocumentMut>()
                .map_err(|error| FileActionError::InvalidTemplate(error.to_string()))?;
            let section = document.get_mut(table).and_then(toml_edit::Item::as_table_mut)
                .ok_or_else(|| FileActionError::InvalidTemplate(format!("Missing TOML table [{table}]")))?;
            section.insert(key, toml_edit::value(data));
            fs::write(output_path, document.to_string())
                .map_err(|_| FileActionError::FailedToWriteFile(output_path.display().to_string()))?;
        }
        TemplateEdit::InsertAfter { target, content, new_line } => {
            let data = resolve_template_value(content, config, windows)?;
            file_action(FileAction::Insert {
                file: output_path,
                target,
                new_content: &data,
                is_new_line: *new_line,
            })?;
        }
        TemplateEdit::Replace { replacement } => {
            let data = resolve_template_value(replacement, config, windows)?;
            file_action(FileAction::Replace {
                file: output_path,
                new_content: &data,
            })?;
        }
    }
    Ok(())
}
#[cfg(test)]
mod firmware_target_tests {
    use super::*;
    use crate::firmware::firmware_store::FIRMWARE_TEMPLATE_LIST;
    use std::process::Command;
    use uuid::Uuid;

    struct Scratch(std::path::PathBuf);
    impl Scratch {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!("pinora-target-test-{}", Uuid::new_v4()));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
    }

    fn config(id: &str) -> ProjectConfig {
        ProjectConfig {
            project_name: "test_project".into(),
            firmware_path: "deep/project/Firmware".into(),
            ui_path: "deep/project/UI".into(),
            id: id.into(),
            build_command: "just build-firmware".into(),
            flash_command: "just flash".into(),
            install_components: vec![],
        }
    }

    fn render(source: &str, config: &ProjectConfig, windows: bool) -> String {
        let temp = Scratch::new();
        let path = temp.0.join("config.toml");
        fs::write(&path, source).unwrap();
        for edit in FIRMWARE_TEMPLATE_LIST[0].edits {
            apply_edit_for_platform(&path, edit, config, windows).unwrap();
        }
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn windows_targets_are_short_stable_and_isolated() {
        let a = config("8f31a240-1234-4abc-8abc-012345678901");
        let b = config("b772c119-5678-4abc-8abc-012345678901");
        assert_eq!(a.firmware_target_dir(true).unwrap(), "C:/p/wo10j");
        assert_eq!(b.firmware_target_dir(true).unwrap(), "C:/p/2ucaa");
        assert_ne!(a.firmware_target_dir(true).unwrap(), b.firmware_target_dir(true).unwrap());
        let mut renamed = a.clone();
        renamed.project_name = "renamed".into();
        renamed.firmware_path = "another/very/deep/Firmware".into();
        assert_eq!(a.firmware_target_dir(true).unwrap(), renamed.firmware_target_dir(true).unwrap());
        let firmware_target_dir = a.firmware_target_dir(true).unwrap();
        assert!(firmware_target_dir.len() <= 10);
        assert_eq!(firmware_target_dir.len(), 10);
        assert!(firmware_target_dir.starts_with("C:/p/"));
        assert!(firmware_target_dir[5..].bytes().all(|b| b.is_ascii_digit() || b.is_ascii_lowercase()));
        assert_eq!(firmware_target_dir, a.firmware_target_dir(true).unwrap());
        // Same old 12-hex prefix, different final byte: the full UUID affects the key.
        let c = config("8f31a240-1234-4abc-8abc-012345678902");
        assert_eq!(c.firmware_target_dir(true).unwrap(), "C:/p/ti312");
        assert_ne!(firmware_target_dir, c.firmware_target_dir(true).unwrap());
        for project in [&a, &b, &c] {
            let path = project.firmware_target_dir(true).unwrap();
            assert!(path.len() <= 10);
            assert!(path.starts_with("C:/p/"));
        }
        assert_eq!(a.firmware_target_dir(false).unwrap(), "target");
    }

    #[test]
    fn colliding_keys_are_rejected_and_same_owner_can_reuse() {
        use crate::project_config::claim_firmware_target;
        let a = config("e99c77f8-4ff4-4996-8e24-e27edb5adc9d");
        let b = config("4759cd7c-f18e-471b-927b-6598419267e4");
        // A real collision in the 36^5 namespace, pinned as a regression case.
        assert_eq!(a.firmware_target_dir(true).unwrap(), "C:/p/nmmnv");
        assert_eq!(a.firmware_target_dir(true).unwrap(), b.firmware_target_dir(true).unwrap());
        let temp = Scratch::new();
        let target = temp.0.join("nmmnv");
        let owner = Uuid::parse_str(&a.id).unwrap();
        claim_firmware_target(&target, owner).unwrap();
        claim_firmware_target(&target, owner).unwrap();
        fs::create_dir(&target).unwrap();
        fs::write(target.join("artifact"), "keep").unwrap();
        let error = claim_firmware_target(&target, Uuid::parse_str(&b.id).unwrap()).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
        assert!(error.to_string().contains("Refusing to share"));
        assert_eq!(fs::read_to_string(target.with_extension("owner")).unwrap().trim(), a.id);
        assert_eq!(fs::read_to_string(target.join("artifact")).unwrap(), "keep");
        claim_firmware_target(&target, owner).unwrap();
    }

    #[test]
    fn unowned_targets_and_corrupt_owner_files_are_not_reused() {
        use crate::project_config::claim_firmware_target;
        let temp = Scratch::new();
        let target = temp.0.join("abc12");
        fs::create_dir(&target).unwrap();
        assert!(claim_firmware_target(&target, Uuid::new_v4()).is_err());
        assert!(!target.with_extension("owner").exists());
        fs::write(target.with_extension("owner"), "incomplete").unwrap();
        assert!(claim_firmware_target(&target, Uuid::new_v4()).is_err());
        assert_eq!(fs::read_to_string(target.with_extension("owner")).unwrap(), "incomplete");
    }
    #[test]
    fn uuid_components_are_canonical_and_invalid_ids_are_rejected() {
        let upper = config("B772C119-5678-4ABC-8ABC-012345678901");
        assert_eq!(upper.firmware_target_dir(true).unwrap(), "C:/p/2ucaa");
        for id in ["", "../other", "C:/t", "../../b772c119-5678-4abc-8abc-012345678901", "bad\"\nkey = 1"] {
            assert!(config(id).firmware_target_dir(true).is_err(), "{id}");
        }
        let saved = serde_json::to_string(&upper).unwrap();
        let loaded: ProjectConfig = serde_json::from_str(&saved).unwrap();
        assert_eq!(upper.firmware_target_dir(true).unwrap(), loaded.firmware_target_dir(true).unwrap());
    }

    #[test]
    fn generated_config_preserves_firmware_settings_on_both_platforms() {
        let fixture = include_str!("firmware/fixtures/config.toml");
        let project = config("8f31a240-1234-4abc-8abc-012345678901");
        for windows in [true, false] {
            // Support the updated master, the published legacy template, and a missing key.
            for source in [
                fixture.to_string(),
                fixture.replace("target-dir = \"target\"", "target-dir = \"C:/t\""),
                fixture.replace("target-dir = \"target\"", "target-dir = \"C:/pt/7ce02822d803\""),
                fixture.replace("target-dir = \"target\"", ""),
            ] {
                let rendered = render(&source, &project, windows);
                let actual = rendered.parse::<toml_edit::DocumentMut>().unwrap();
                let before = source.parse::<toml_edit::DocumentMut>().unwrap();
                assert_eq!(actual["build"]["target-dir"].as_str().unwrap(), project.firmware_target_dir(windows).unwrap());
                assert_eq!(actual["build"]["target"].as_str(), before["build"]["target"].as_str());
                for section in ["target", "unstable", "env"] {
                    assert_eq!(actual[section].to_string(), before[section].to_string());
                }
                assert_eq!(render(&rendered, &project, windows), rendered);
            }
        }
    }

    #[test]
    fn malformed_config_or_id_fails_without_rewriting_file() {
        let temp = Scratch::new();
        let path = temp.0.join("config.toml");
        let valid = config("8f31a240-1234-4abc-8abc-012345678901");
        for (source, project) in [
            ("[build", valid.clone()),
            ("[env]\nMCU = 'esp32'", valid),
            ("[build]\ntarget-dir = 'target'", config("../unsafe")),
        ] {
            fs::write(&path, source).unwrap();
            assert!(apply_edit_for_platform(&path, &FIRMWARE_TEMPLATE_LIST[0].edits[0], &project, true).is_err());
            assert_eq!(fs::read_to_string(&path).unwrap(), source);
        }
    }

    #[tokio::test]
    async fn download_and_scaffold_two_projects() {
        use std::io::{Read, Write};
        // Exercise the actual download + template-edit pipeline without GitHub availability.
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let url: &'static str = Box::leak(format!("http://{}/config.toml", listener.local_addr().unwrap()).into_boxed_str());
        let server = std::thread::spawn(move || {
            for _ in 0..2 {
                let (mut stream, _) = listener.accept().unwrap();
                stream.set_read_timeout(Some(std::time::Duration::from_secs(10))).unwrap();
                let mut request = [0; 4096];
                stream.read(&mut request).unwrap();
                let body = include_str!("firmware/fixtures/config.toml");
                write!(stream, "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).unwrap();
            }
        });
        let root = Scratch::new();
        let source = SourceTemplate {
            name: ".cargo/config.toml",
            output_path: ".cargo/config.toml",
            github_source_url: url,
            edits: FIRMWARE_TEMPLATE_LIST[0].edits,
        };
        let mut targets = vec![];
        for name in ["test1", "test2"] {
            let project = config(&Uuid::new_v4().to_string());
            let firmware = root.0.join(name).join("Firmware");
            generate_file(&source, &firmware, &project).await.unwrap();
            let text = fs::read_to_string(firmware.join(".cargo/config.toml")).unwrap();
            let document = text.parse::<toml_edit::DocumentMut>().unwrap();
            assert_eq!(document["build"]["target-dir"].as_str().unwrap(), project.firmware_target_dir(cfg!(windows)).unwrap());
            let target = project.firmware_target_dir(true).unwrap();
            assert!(target.len() <= 10);
            assert!(target.starts_with("C:/p/"));
            if cfg!(windows) {
                let marker = Path::new(&target).with_extension("owner");
                assert_eq!(fs::read_to_string(&marker).unwrap().trim(), project.id);
                // Remove only this test's own reservation; no artifacts were built.
                fs::remove_file(marker).unwrap();
            }
            targets.push(target);
        }
        server.join().unwrap();
        assert_ne!(targets[0], targets[1]);
    }

    #[test]
    fn cargo_metadata_and_clean_use_only_the_owned_target() {
        let source = Scratch::new();
        let mut projects = vec![];
        let mut external_targets = vec![];
        for name in ["test1", "test2"] {
            let project = config(&Uuid::new_v4().to_string());
            let root = source.0.join(name);
            fs::create_dir_all(root.join(".cargo")).unwrap();
            fs::create_dir(root.join("src")).unwrap();
            fs::write(root.join("src/lib.rs"), "").unwrap();
            fs::write(root.join("Cargo.toml"), format!("[package]\nname = '{name}'\nversion = '0.1.0'\nedition = '2021'\n[workspace]\n")).unwrap();
            // A host crate tests real Cargo resolution/clean without compiling ESP-IDF.
            fs::write(root.join(".cargo/config.toml"), render("[build]\ntarget-dir = 'target'\n", &project, cfg!(windows))).unwrap();
            let target = if cfg!(windows) {
                std::path::PathBuf::from(project.firmware_target_dir(true).unwrap())
            } else { root.join("target") };
            crate::project_config::claim_firmware_target(&target, Uuid::parse_str(&project.id).unwrap()).unwrap();
            // Never claim or clean any pre-existing build directory.
            fs::create_dir(&target).expect("test target must not already exist");
            fs::write(target.join("isolation-marker"), name).unwrap();
            if cfg!(windows) { external_targets.push(target.clone()); }
            let output = Command::new(env!("CARGO")).args(["metadata", "--no-deps", "--format-version", "1", "--offline"])
                .env_remove("CARGO_TARGET_DIR").env_remove("CARGO_BUILD_TARGET_DIR")
                .current_dir(&root).output().unwrap();
            assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
            let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            let resolved = std::path::PathBuf::from(metadata["target_directory"].as_str().unwrap());
            assert_eq!(fs::canonicalize(&resolved).unwrap(), fs::canonicalize(&target).unwrap());
            projects.push((root, target));
        }
        let output = Command::new(env!("CARGO")).args(["clean", "--offline"])
            .env_remove("CARGO_TARGET_DIR").env_remove("CARGO_BUILD_TARGET_DIR")
            .current_dir(&projects[0].0).output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        assert!(!projects[0].1.join("isolation-marker").exists());
        assert!(projects[1].1.join("isolation-marker").exists());
        let output = Command::new(env!("CARGO")).args(["clean", "--offline"])
            .env_remove("CARGO_TARGET_DIR").env_remove("CARGO_BUILD_TARGET_DIR")
            .current_dir(&projects[1].0).output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        for (_, target) in &projects {
            let marker = target.with_extension("owner");
            assert!(marker.is_file(), "Cargo clean must preserve the UUID reservation");
            fs::remove_file(marker).unwrap();
        }
        for target in external_targets { assert!(!target.exists()); }
    }
}
