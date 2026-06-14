
use crate::sharedTypes::{TemplateFile, BRANCH_NAME};
use std::path::Path;
use anyhow::{ Result};
use anyhow::{Context};
use tokio;

pub async fn load_files_json(path: &Path) -> Result<Vec<TemplateFile>> {
    let json_text = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("Failed to read JSON file: {}", path.display()))?;

    let files: Vec<TemplateFile> =
        serde_json::from_str(&json_text)
        .with_context(|| "Failed to parse files.json")?
        ;
    let valid_files: Vec<TemplateFile> = files
        .into_iter()
        .map(|mut item| {
            let is_main_branch = item.source_url.contains("/main/");
            if is_main_branch && BRANCH_NAME != "main" {
                let updated_url = item.source_url.replace("/main/", &format!("/{}/", BRANCH_NAME));
                item.source_url = updated_url;
               
            }
            item
        })
        .collect();

    Ok(valid_files)
}

