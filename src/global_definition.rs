use serde::{Deserialize, Serialize};

pub static BRANCH_NAME: &str = "v0"; 

pub  struct  CargoDependency {
    pub name: &'static str,
    pub version: &'static str,
     pub features: &'static [&'static str]
}

pub  struct  Template {
    pub name: &'static str,
    pub source_path: &'static str,
    pub output_path: &'static str,
    // pub swap:bool,
    // pub replacement: &'static str
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