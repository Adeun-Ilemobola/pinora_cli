use crate::{
    global_definition::{CargoDependency, Template},
    ui::ui_definition::NodeDependency,
};

macro_rules! ui_template {
    ($path:literal) => {
        Template {
            name: $path,
            source_path: concat!(
                "https://raw.githubusercontent.com/Adeun-Ilemobola/tauri_esp_app/main/",
                $path
            ),
            output_path: $path,
        }
    };
}

pub static UI_TEMPLATE_LIST: [Template; 39] = [
    ui_template!("vite.config.ts"),
    ui_template!("tsconfig.node.json"),
    ui_template!("tsconfig.json"),
    ui_template!("components.json"),
    ui_template!("src/main.tsx"),
    ui_template!("src/Layout.tsx"),
    ui_template!("src/App.css"),

    ui_template!("src/components/Modules/ModuleCore.tsx"),
    ui_template!("src/components/Modules/ButtonModule.tsx"),
    ui_template!("src/components/Modules/led.tsx"),

    // ui_template!("src/components/Modules/Rangefinder.tsx"),
    // ui_template!("src/components/Modules/Servo.tsx"),
    // ui_template!("src/components/Grid.tsx"),

    ui_template!("src/components/LogFrame.tsx"),
    ui_template!("src/components/PortInput.tsx"),
    ui_template!("src/components/theme-provider.tsx"),
    ui_template!("src/hooks/use-mobile.ts"),
    ui_template!("src/lib/Modules/BUTTON.ts"),
    ui_template!("src/lib/Modules/LED.ts"),

    // ui_template!("src/lib/Modules/LEDCLUSTER.ts"),
    // ui_template!("src/lib/Modules/LIDAR.ts"),
    // ui_template!("src/lib/Modules/RANGEFINDER.ts"),
    // ui_template!("src/lib/Modules/SERVO.ts"),
    
    ui_template!("src/lib/ListenStore.ts"),
    ui_template!("src/lib/logging.ts"),
    ui_template!("src/lib/ModuleCommand.ts"),
    ui_template!("src/lib/ModuleDefinitionSchema.ts"),
    ui_template!("src/lib/ModuleEven.ts"),
    ui_template!("src/lib/ModuleStore.ts"),
    ui_template!("src/lib/utils.ts"),
    ui_template!("src/page/App.tsx"),
    ui_template!("src/page/Dashboard.tsx"),
    ui_template!("src/page/Devices.tsx"),
    ui_template!("src/page/Logs.tsx"),
    ui_template!("src/page/NotFound.tsx"),
    ui_template!("src/page/PortSettings.tsx"),
    ui_template!("src-tauri/src/protocol/command.rs"),
    ui_template!("src-tauri/src/protocol/global_definition_protocol.rs"),
    ui_template!("src-tauri/src/protocol/mod.rs"),
    ui_template!("src-tauri/src/protocol/module_event.rs"),
    ui_template!("src-tauri/src/protocol/registration.rs"),
    ui_template!("src-tauri/src/shared_types/mod.rs"),
    ui_template!("src-tauri/src/shared_types/state.rs"),
    ui_template!("src-tauri/src/global_definition.rs"),
    ui_template!("src-tauri/src/lib.rs"),
    ui_template!("src-tauri/src/main.rs"),
];

pub static UI_DEPENDENCY_LIST: [NodeDependency; 35] = [
    // Runtime dependencies
    NodeDependency { name: "@fontsource-variable/eb-garamond", version: "^5.2.7", dev: false },
    NodeDependency { name: "@fontsource-variable/figtree", version: "^5.2.10", dev: false },
    NodeDependency { name: "@fontsource-variable/geist", version: "^5.2.9", dev: false },
    NodeDependency { name: "@fontsource-variable/jetbrains-mono", version: "^5.2.8", dev: false },
    NodeDependency { name: "@fontsource-variable/roboto-slab", version: "^5.2.8", dev: false },
    NodeDependency { name: "@phosphor-icons/react", version: "^2.1.10", dev: false },
    NodeDependency { name: "@tailwindcss/vite", version: "^4.3.1", dev: false },
    NodeDependency { name: "@tanstack/react-virtual", version: "^3.14.3", dev: false },
    NodeDependency { name: "@tauri-apps/api", version: "^2", dev: false },
    NodeDependency { name: "@tauri-apps/plugin-log", version: "~2", dev: false },
    NodeDependency { name: "@tauri-apps/plugin-opener", version: "^2", dev: false },
    NodeDependency { name: "class-variance-authority", version: "^0.7.1", dev: false },
    NodeDependency { name: "clsx", version: "^2.1.1", dev: false },
    NodeDependency { name: "cmdk", version: "^1.1.1", dev: false },
    NodeDependency { name: "lucide-react", version: "^1.23.0", dev: false },
    NodeDependency { name: "next-themes", version: "^0.4.6", dev: false },
    NodeDependency { name: "radix-ui", version: "^1.5.0", dev: false },
    NodeDependency { name: "react", version: "^19.1.0", dev: false },
    NodeDependency { name: "react-dom", version: "^19.1.0", dev: false },
    NodeDependency { name: "react-router-dom", version: "^7.17.0", dev: false },
    NodeDependency { name: "shadcn", version: "^4.11.0", dev: false },
    NodeDependency { name: "sonner", version: "^2.0.7", dev: false },
    NodeDependency { name: "tailwind-merge", version: "^3.6.0", dev: false },
    NodeDependency { name: "tailwindcss", version: "^4.3.1", dev: false },
    NodeDependency { name: "tw-animate-css", version: "^1.4.0", dev: false },
    NodeDependency { name: "vaul", version: "^1.1.2", dev: false },
    NodeDependency { name: "zod", version: "^4.4.3", dev: false },
    NodeDependency { name: "zustand", version: "^5.0.14", dev: false },

    // Development dependencies
    NodeDependency { name: "@tauri-apps/cli", version: "^2", dev: true },
    NodeDependency { name: "@types/node", version: "^25.9.3", dev: true },
    NodeDependency { name: "@types/react", version: "^19.1.8", dev: true },
    NodeDependency { name: "@types/react-dom", version: "^19.1.6", dev: true },
    NodeDependency { name: "@vitejs/plugin-react", version: "^4.6.0", dev: true },
    NodeDependency { name: "typescript", version: "~5.8.3", dev: true },
    NodeDependency { name: "vite", version: "^7.0.4", dev: true },
];

/// Components installed through the shadcn CLI.
pub static SHADCN_COMPONENT_LIST: [&str; 24] = [
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
    "slider",
    "sonner",
    "spinner",
    "textarea",
    "tooltip",
];

pub static TAURI_DEPENDENCY_LIST: [CargoDependency; 7] = [
    CargoDependency {
        name: "tauri",
        version: "2",
        features: &[],
    },
    CargoDependency {
        name: "tauri-plugin-opener",
        version: "2",
        features: &[],
    },
    CargoDependency {
        name: "serde",
        version: "1",
        features: &["derive"],
    },
    CargoDependency {
        name: "serde_json",
        version: "1",
        features: &[],
    },
    CargoDependency {
        name: "serialport",
        version: "4",
        features: &[],
    },
    CargoDependency {
        name: "tauri-plugin-log",
        version: "2",
        features: &[],
    },
    CargoDependency {
        name: "log",
        version: "0.4",
        features: &[],
    },
];