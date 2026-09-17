use serde::{Deserialize, Serialize};
use std::fmt;

pub static BRANCH_NAME: &str = "v0"; 

#[derive(Debug, Serialize, Clone)]
pub struct SourceTemplate {
    pub name: &'static str,
    pub github_source_url: &'static str,
    pub output_path: &'static str,
    pub edits: &'static [TemplateEdit],
}

impl fmt::Display for SourceTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SourceTemplate {{ name: {}, github_source_url: {}, output_path: {}, edits: {:?} }}",
            self.name,
            self.github_source_url,
            self.output_path,
            self.edits
        )
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum TemplateEdit {
    /// Set a string key without depending on its previous spelling or value.
    SetTomlString {
        table: &'static str,
        key: &'static str,
        value: TemplateValue,
    },
    Replace {
        // target: &'static str,
        replacement: TemplateValue,
    },
    InsertAfter {
        target: &'static str,
        content: TemplateValue,
        new_line:bool
    },
}

impl fmt::Display for TemplateEdit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateEdit::SetTomlString { table, key, value } => {
                write!(f, "SetTomlString {{ table: {}, key: {}, value: {:?} }}", table, key, value)
            },
            TemplateEdit::Replace { replacement } => {
                write!(f, "Replace {{ replacement: {:?} }}", replacement)
            },
            TemplateEdit::InsertAfter { target, content, new_line } => {
                write!(f, "InsertAfter {{ target: {}, content: {:?}, new_line: {} }}", target, content, new_line)
            },
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum TemplateValue {
    Literal(&'static str),
    ProjectName,
    FirmwareTargetDir,
    FirmwarePath,
    UiPath,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubItem {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub download_url: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProjectConfig {
    pub project_name: String,
    // pub project_path: String,
    pub firmware_path:String,
    pub  ui_path:String,
    pub id: String,
    pub build_command: String,
    pub flash_command: String,
    pub install_components: Vec<String>,
}

impl fmt::Display for ProjectConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProjectConfig {{ project_name: {}, firmware_path: {}, ui_path: {}, id: {}, build_command: {}, flash_command: {}, install_components: {:?} }}",
            self.project_name,
            self.firmware_path,
            self.ui_path,
            self.id,
            self.build_command,
            self.flash_command,
            self.install_components
        )
    }
}



pub enum LogType {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Serialize, Clone, Copy, Deserialize)]
pub enum ProgressType {
    Started,
    Step,
    Finished,
    Failed,
    Complete
}

/// One progress event. Every task emits exactly one `Started` and exactly one terminal
/// event (`Finished` or `Failed`), so a consumer can always close out a task it opened.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProgressLogShape {
    pub task: String,
    pub stage: ProgressType,
    pub message: String,
    pub detail: Option<String>,
    pub step: u32,
    pub total: u32,
    
}