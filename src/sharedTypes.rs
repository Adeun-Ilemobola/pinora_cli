use serde::{Deserialize, Serialize};

pub  static UI_FOLDER_NAME:&str = "UI";
pub  static ESP_FOLDER_NAME:&str = "Firmware";


// #[derive(Debug, Deserialize)]
// pub struct TemplateFile {
//     pub name: Option<String>,
//     pub source_url: String,
//     pub output_path: String,
// }

pub  struct  CargoDependency {
    pub name: &'static str,
    pub version: &'static str,
     pub features: &'static [&'static str]
}


pub static FIRMWARE_DEPENDENCY_LIST: [CargoDependency; 6] =[
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
        name: "log",
        version: "0.4",
        features: &[],
    },
    CargoDependency{
        name: "pwm-pca9685",
        version: "1",
        features: &[],
    },
     CargoDependency{
        name: "uuid",
        version: "1.23.2",
        features: &["v4"],
    },
];

pub static TAURI_DEPENDENCY_LIST: [CargoDependency; 6] = [
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
    // required by the `tauri_plugin_log` builder registered in the template's lib.rs
    CargoDependency{
        name: "tauri-plugin-log",
        version: "2",
        features: &[],
    },
];

pub struct NodeDependency {
    pub name: &'static str,
    pub version: &'static str,
    pub dev: bool,
}

/// Packages the UI template needs on top of what `bun create tauri-app --template react-ts`
/// already scaffolds (react, react-dom, vite, typescript, @tauri-apps/{api,cli,plugin-opener}).
pub static UI_DEPENDENCY_LIST: [NodeDependency; 24] = [
    // Tailwind v4 + shadcn runtime
    NodeDependency { name: "tailwindcss", version: "^4.3.1", dev: false },
    NodeDependency { name: "@tailwindcss/vite", version: "^4.3.1", dev: false },
    NodeDependency { name: "tw-animate-css", version: "^1.4.0", dev: false },
    NodeDependency { name: "shadcn", version: "^4.11.0", dev: false },
    NodeDependency { name: "radix-ui", version: "^1.5.0", dev: false },
    NodeDependency { name: "class-variance-authority", version: "^0.7.1", dev: false },
    NodeDependency { name: "clsx", version: "^2.1.1", dev: false },
    NodeDependency { name: "tailwind-merge", version: "^3.6.0", dev: false },
    NodeDependency { name: "cmdk", version: "^1.1.1", dev: false },
    NodeDependency { name: "vaul", version: "^1.1.2", dev: false },
    NodeDependency { name: "sonner", version: "^2.0.7", dev: false },
    NodeDependency { name: "next-themes", version: "^0.4.6", dev: false },

    // Icons + fonts
    NodeDependency { name: "@phosphor-icons/react", version: "^2.1.10", dev: false },
    NodeDependency { name: "lucide-react", version: "^1.23.0", dev: false },
    NodeDependency { name: "@fontsource-variable/geist", version: "^5.2.9", dev: false },
    NodeDependency { name: "@fontsource-variable/jetbrains-mono", version: "^5.2.8", dev: false },
    NodeDependency { name: "@fontsource-variable/figtree", version: "^5.2.10", dev: false },
    NodeDependency { name: "@fontsource-variable/roboto-slab", version: "^5.2.8", dev: false },
    NodeDependency { name: "@fontsource-variable/eb-garamond", version: "^5.2.7", dev: false },

    // App runtime
    NodeDependency { name: "react-router-dom", version: "^7.17.0", dev: false },
    NodeDependency { name: "zustand", version: "^5.0.14", dev: false },
    NodeDependency { name: "zod", version: "^4.4.3", dev: false },
    NodeDependency { name: "@tanstack/react-virtual", version: "^3.14.3", dev: false },
    NodeDependency { name: "@tauri-apps/plugin-log", version: "~2", dev: false },
];

/// shadcn components pulled in by the UI template. Fetched with the shadcn CLI rather than
/// downloaded, so they follow the style/baseColor in the template's components.json.
pub static SHADCN_COMPONENT_LIST: [&str; 23] = [
    "badge",
    "button",
    "button-group",
    "card",
    "collapsible",
    "command",
    "context-menu",
    "dialog",
    "drawer",
    "dropdown-menu",
    "input",
    "input-group",
    "label",
    "scroll-area",
    "select",
    "separator",
    "sheet",
    "sidebar",
    "skeleton",
    "sonner",
    "spinner",
    "textarea",
    "tooltip",
];
pub  struct  Template {
    pub name: &'static str,
    pub source_path: &'static str,
    pub output_path: &'static str,
    // pub swap:bool,
    // pub replacement: &'static str
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
            // swap: false,
            // replacement: "",
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
    firmware_template!("src/utilities/moduleconflg.rs"),
    firmware_template!("src/utilities/serdeprotocol.rs"),

    // Firmware entry point
    firmware_template!("src/main.rs"),
];
macro_rules! ui_template {
    ($path:literal) => {
        Template {
            name: $path,
            source_path: concat!(
                "https://raw.githubusercontent.com/Adeun-Ilemobola/tauri_esp_app/main/",
                $path
            ),
            output_path:  $path,
            // swap: false,
            // replacement: "",
        }
    };
}
pub static UI_TEMPLATE_LIST: [Template; 29] = [
    ui_template!("vite.config.ts"),
    ui_template!("tsconfig.node.json"),
    ui_template!("tsconfig.json"),
    ui_template!("components.json"),

    ui_template!("src/main.tsx"),
    ui_template!("src/Layout.tsx"),
    ui_template!("src/App.css"),

    ui_template!("src/components/LogFrame.tsx"),
    ui_template!("src/components/theme-provider.tsx"),

    // Hardware modules
    ui_template!("src/components/Modules/ModuleCore.tsx"),
    ui_template!("src/components/Modules/ButtonModule.tsx"),
    ui_template!("src/components/Modules/led.tsx"),
    // ui_template!("src/components/Modules/Servo.tsx"),

    ui_template!("src/Hook/state.ts"),
    ui_template!("src/Hook/Zod.ts"),
    ui_template!("src/Hook/Command.ts"),
    ui_template!("src/Hook/Event.ts"),

    ui_template!("src/hooks/use-mobile.ts"),

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
// #[derive(Serialize)]
// pub struct ProgressEvent<'a> {
//     pub stage: &'a str,
//     pub message: &'a str,
//     pub current: u8,
//     pub total: u8,
// }



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

// #[derive(Debug, Clone)]
// pub struct UISourceFile {
//     pub source_url: String,
//     pub output_path: String,
// }

// #[derive(Debug, Clone)]
// pub struct UIReplacement {
//     pub source_url: String,
//     pub target_path: String,
// }
