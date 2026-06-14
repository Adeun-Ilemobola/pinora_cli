use crate::utility::{download_file, emit_progress, log};
use crate::sharedTypes::{ProjectConfig, LogType , TemplateFile};
use crate::file_json::load_files_json;
use std::path::{ PathBuf};
use std::process::Command;


pub fn load_project_database() -> Option<Vec<ProjectConfig>> {
    let projects: Vec<ProjectConfig> = Vec::new();

    let home_dir = dirs::home_dir().expect("Failed to get home directory");

    let db_path = home_dir.join("esp_rust_projects.json");

    if !db_path.exists() {
        log(
            "Project database file does not exist. Creating a new one.",
            "Project Database",
            LogType::Warning,
        );
        std::fs::write(&db_path, "[]").expect("Failed to create project database file");
        return Some(projects);
    }
    let db_content =
        std::fs::read_to_string(&db_path).expect("Failed to read project database file");
    let parsed_projects: Vec<ProjectConfig> =
        serde_json::from_str(&db_content).expect("Failed to parse project database file");
    Some(parsed_projects)
}

pub fn save_project_to_database(project_config: &ProjectConfig) {
    let mut projects = load_project_database().expect("Failed to load project database");
    projects.push(project_config.clone());
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let db_path = home_dir.join("esp_rust_projects.json");
    std::fs::write(
        &db_path,
        serde_json::to_string_pretty(&projects).expect("Failed to serialize project database"),
    )
    .expect("Failed to write project database file");
    log(
        "Project saved to database successfully!",
        "Project Database",
        LogType::Info,
    );
}



pub  fn update_project_config(new_config: &ProjectConfig){
    let get_projects = load_project_database().expect("Failed to load project database");
    let  updated_projects: Vec<ProjectConfig> = get_projects.into_iter()
        .map(|proj| if proj.id == new_config.id { new_config.clone() } else { proj })
        .collect();
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let db_path = home_dir.join("esp_rust_projects.json");
    std::fs::write(
        &db_path,
        serde_json::to_string_pretty(&updated_projects).expect("Failed to serialize project database"),
    )
    .expect("Failed to write project database file");
    log(
        "Project saved to database successfully!",
        "Project Database",
        LogType::Info,
    );
}

pub async fn update_project_struct(project_path: &std::path::PathBuf, max_steps: u8) -> bool {
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let files_json_path = manifest_path.join("src").join("files.json");

    let files: Vec<TemplateFile> = load_files_json(&files_json_path).await.unwrap();
    log(
        &format!("Updating project structure with {} files...", files.len()),
        "Project Structure Update",
        LogType::Info,
    );

    for (i, file) in files.iter().enumerate() {
        emit_progress(
            "Project Structure Update",
            &format!(
                "Processing file: {}",
                file.name.as_ref().unwrap_or(&file.output_path)
            ),
            (i + 1) as u8,
            max_steps,
        );
        let output_path = project_path.join(&file.output_path);

        log(
            &format!(
                "
            Processing file: {}
            file source: {}
            file output path: {}
            name: {}
            ",
                output_path.display(),
                file.source_url,
                output_path.display(),
                file.name.as_ref().unwrap_or(&file.output_path)
            ),
            "Project Structure Update",
            LogType::Info,
        );
        match download_file(&file.source_url, &output_path).await {
            Ok(_) => log(
                &format!("File created: {}", output_path.display()),
                "Project Structure Update",
                LogType::Info,
            ),
            Err(e) => log(
                &format!("Failed to create file {}: {}", output_path.display(), e),
                "Project Structure Update",
                LogType::Error,
            ),
        }
    }

    let dependency_to_add_list = vec![
        ("uuid", None),
        ("serde", Some("derive")),
        ("serde_json", None),
        ("anyhow", None),
    ];

    for (dep, feature) in dependency_to_add_list {
        let mut cmd = Command::new("cargo");
        cmd.arg("add").arg(dep).current_dir(&project_path);
        if let Some(feature_name) = feature {
            cmd.arg("--features").arg(feature_name);
        }
        match cmd.status() {
            Ok(status) if status.success() => log(
                &format!("Dependency '{}' added successfully.", dep),
                "Project Structure Update",
                LogType::Info,
            ),
            Ok(status) => log(
                &format!(
                    "Failed to add dependency '{}'. Exit status: {}",
                    dep, status
                ),
                "Project Structure Update",
                LogType::Error,
            ),
            Err(e) => log(
                &format!("Failed to run cargo add for '{}': {}", dep, e),
                "Project Structure Update",
                LogType::Error,
            ),
        };
    }

    emit_progress(
        "Project Structure Update",
        "Updating Cargo.toml with dependencies...",
        (79) as u8,
        max_steps,
    );
    Command::new("cargo")
        .arg("add")
        .arg("serde_json")
        .current_dir(&project_path)
        .status()
        .expect("Failed to run cargo add");

    let cargo_toml_path = project_path.join("Cargo.toml");
    if cargo_toml_path.exists() {
        let mut cargo_toml_content =
            std::fs::read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml");
        // remove old dependencies if they exist {""}
        let marker = "[dependencies]";
        let dependency_to_add = r#"uuid = { version = "1.23.2", features = ["v4"] }"#;
        if let Some(index) = cargo_toml_content.find(marker) {
            // remove old dependencies if they exist {"uuid = "1.20.0"
            let old_dependency_line = cargo_toml_content
                .lines()
                .find(|line| line.contains(r#"uuid = "1.20.0"#));
            if let Some(old_line) = old_dependency_line {
                cargo_toml_content = cargo_toml_content.replace(old_line, "");
            }

            let insert_position = index + marker.len();
            cargo_toml_content.insert_str(insert_position, &format!("\n{}", dependency_to_add));
        }

        std::fs::write(&cargo_toml_path, cargo_toml_content).expect("Failed to update Cargo.toml");
        log(
            "Cargo.toml updated with ESP dependencies.",
            "Project Structure Update",
            LogType::Info,
        );
    }
    return true;
}