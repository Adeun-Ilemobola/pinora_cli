use serde::{Deserialize, Serialize};

pub  static UI_FOLDER_NAME:&str = "UI";
pub  static ESP_FOLDER_NAME:&str = "Firmware";


#[derive(Debug, Deserialize)]
pub struct TemplateFile {
    pub name: Option<String>,
    pub source_url: String,
    pub output_path: String,
}

pub  struct  CargoDependency {
    pub name: &'static str,
    pub version: &'static str,
     pub features: &'static [&'static str]
}

pub static TAURI_DEPENDENCY_LIST: [CargoDependency; 5] = [
    CargoDependency {
        name: "anyhow",
        version: "1",
        features: &[],
    },
    CargoDependency{
        name: "serde",
        version: "1",
        features: &["derive"],
    },
    CargoDependency{
        name: "serde_json",
        version: "1",
        features: &[],
    },
    CargoDependency{
        name: "serialport",
        version: "4",
        features: &[],
    },
    CargoDependency{
        name: "log",
        version: "0.4",
        features: &[],
    },
];
pub  struct  Template {
    pub name: &'static str,
    pub source_path: &'static str,
    pub output_path: &'static str,
    pub swap:bool,
    pub replacement: &'static str
}
macro_rules! ui_template {
    ($path:literal) => {
        Template {
            name: $path,
            source_path: concat!(
                "https://raw.githubusercontent.com/esp-rust/esp-rust-templates/v0/ui/",
                $path
            ),
            output_path: concat!("/", $path),
            swap: false,
            replacement: "",
        }
    };
}




macro_rules! firmware_template {
    ($path:literal) => {
        Template {
            name: $path,
            source_path: concat!(
                "https://raw.githubusercontent.com/Adeun-Ilemobola/rust_esp32_based/v0/",
                $path
            ),
            output_path: $path,
            swap: false,
            replacement: "",
        }
    };
}
pub static FIRMWARE_TEMPLATE_LIST: [Template; 12] = [
    // Core
    firmware_template!("src/core/hardware.rs"),
    firmware_template!("src/core/mod.rs"),
    firmware_template!("src/core/modulecore.rs"),

    // Modules
    firmware_template!("src/module/buttonmodule.rs"),
    firmware_template!("src/module/ledmodule.rs"),
    firmware_template!("src/module/mod.rs"),

    // Utilities
    firmware_template!("src/utilities/logger.rs"),
    firmware_template!("src/utilities/math.rs"),
    firmware_template!("src/utilities/mod.rs"),
    firmware_template!("src/utilities/moduleconfig.rs"),
    firmware_template!("src/utilities/serdeprotocol.rs"),

    // Firmware entry point
    firmware_template!("src/main.rs"),
];

pub static UI_TEMPLATE_LIST: [Template; 23] = [
    ui_template!("vite.config.ts"),
    ui_template!("tsconfig.node.json"),
    ui_template!("tsconfig.json"),
    ui_template!("components.json"),

    ui_template!("src/main.tsx"),
    ui_template!("src/Layout.tsx"),
    ui_template!("src/App.css"),

    ui_template!("src/components/LogFrame.tsx"),
    ui_template!("src/components/theme-provider.tsx"),

    ui_template!("src/Hook/state.ts"),
    ui_template!("src/Hook/Zod.ts"),

    ui_template!("src/lib/logging.ts"),
    ui_template!("src/lib/utils.ts"),

    ui_template!("src/page/App.tsx"),
    ui_template!("src/page/Dashboard.tsx"),
    ui_template!("src/page/Devices.tsx"),
    ui_template!("src/page/Logs.tsx"),
    ui_template!("src/page/NotFound.tsx"),
    ui_template!("src/page/PortSettings.tsx"),

    // TAURI project files
    ui_template!("src-tauri/src/lib.rs"),
    ui_template!("src-tauri/src/shared_types/mod.rs"),
    ui_template!("src-tauri/src/shared_types/event.rs"),
    ui_template!("src-tauri/src/shared_types/command.rs"),
];

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
    Complete,
}
#[derive(Serialize)]
pub struct ProgressEvent<'a> {
    pub stage: &'a str,
    pub message: &'a str,
    pub current: u8,
    pub total: u8,
}
#[derive(Debug, Serialize, Clone, Deserialize)]
 pub enum ProgressType {
     Error,
     Complete,
     Loading,
     Installing,
     Finished
 }
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProgressLogShape {
    pub stage :ProgressType,
    pub message:String,
    pub id:String
}

pub static BRANCH_NAME: &str = "v0"; // can be set to "main" or "dev" depending on which branch you want to pull template files from

#[derive(Debug, Clone)]
pub struct UISourceFile {
    pub source_url: String,
    pub output_path: String,
}

#[derive(Debug, Clone)]
pub struct UIReplacement {
    pub source_url: String,
    pub target_path: String,
}
