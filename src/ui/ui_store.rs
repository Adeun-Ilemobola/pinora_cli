use crate::{
    global_definition::{CargoDependency, SourceTemplate},
    ui::ui_definition::NodeDependency,
};

macro_rules! ui_template {
    ($path:literal) => {
        SourceTemplate {
            name: $path,
            github_source_url: concat!(
                "https://raw.githubusercontent.com/Adeun-Ilemobola/Pinora_Templat/feat/electrobun-architecture/UI_Templates/",
                $path
            ),
            output_path: $path,
            edits: &[],
        }
    };
    ($path:literal, $( $edit:expr ),+ $(,)?) => {
        SourceTemplate {
            name: $path,
            github_source_url: concat!(
                "https://raw.githubusercontent.com/Adeun-Ilemobola/Pinora_Templat/feat/electrobun-architecture/UI_Templates/",
                $path
            ),
            output_path: $path,
            edits: &[
                $( $edit ),+
            ],
        }
    };
}

pub static UI_TEMPLATE_LIST: [SourceTemplate; 68] = [
    ui_template!(".gitignore"),
    ui_template!("README.md"),
    ui_template!("bun.lock"),
    ui_template!("components.json"),
    ui_template!("electrobun.config.ts"),
    ui_template!("llms.txt"),
    ui_template!("package.json"),
    ui_template!("src/Runtime/ModuleStore.ts"),
    ui_template!("src/bun/index.ts"),
    ui_template!("src/mainview/App.tsx"),
    ui_template!("src/mainview/Modules/IMU/definition.ts"),
    ui_template!("src/mainview/Modules/IMU/view.tsx"),
    ui_template!("src/mainview/Modules/Lidar/definition.ts"),
    ui_template!("src/mainview/Modules/Lidar/view.tsx"),
    ui_template!("src/mainview/Modules/button/definition.ts"),
    ui_template!("src/mainview/Modules/button/view.tsx"),
    ui_template!("src/mainview/Modules/led/definition.ts"),
    ui_template!("src/mainview/Modules/led/view.tsx"),
    ui_template!("src/mainview/Modules/rangefinder/definition.ts"),
    ui_template!("src/mainview/Modules/rangefinder/view.tsx"),
    ui_template!("src/mainview/Modules/servo/definition.ts"),
    ui_template!("src/mainview/Modules/servo/view.tsx"),
    ui_template!("src/mainview/Modules/stepper/definition.ts"),
    ui_template!("src/mainview/Modules/stepper/view.tsx"),
    ui_template!("src/mainview/Pages/logs.tsx"),
    ui_template!("src/mainview/components/Grid.tsx"),
    ui_template!("src/mainview/components/ModuleCore.tsx"),
    ui_template!("src/mainview/components/PointInput.tsx"),
    ui_template!("src/mainview/components/app-sidebar.tsx"),
    ui_template!("src/mainview/components/ui/avatar.tsx"),
    ui_template!("src/mainview/components/ui/badge.tsx"),
    ui_template!("src/mainview/components/ui/button.tsx"),
    ui_template!("src/mainview/components/ui/card.tsx"),
    ui_template!("src/mainview/components/ui/collapsible.tsx"),
    ui_template!("src/mainview/components/ui/command.tsx"),
    ui_template!("src/mainview/components/ui/dialog.tsx"),
    ui_template!("src/mainview/components/ui/dropdown-menu.tsx"),
    ui_template!("src/mainview/components/ui/input-group.tsx"),
    ui_template!("src/mainview/components/ui/input.tsx"),
    ui_template!("src/mainview/components/ui/label.tsx"),
    ui_template!("src/mainview/components/ui/progress.tsx"),
    ui_template!("src/mainview/components/ui/select.tsx"),
    ui_template!("src/mainview/components/ui/separator.tsx"),
    ui_template!("src/mainview/components/ui/sheet.tsx"),
    ui_template!("src/mainview/components/ui/sidebar.tsx"),
    ui_template!("src/mainview/components/ui/skeleton.tsx"),
    ui_template!("src/mainview/components/ui/slider.tsx"),
    ui_template!("src/mainview/components/ui/sonner.tsx"),
    ui_template!("src/mainview/components/ui/table.tsx"),
    ui_template!("src/mainview/components/ui/tabs.tsx"),
    ui_template!("src/mainview/components/ui/textarea.tsx"),
    ui_template!("src/mainview/components/ui/tooltip.tsx"),
    ui_template!("src/mainview/electrobun.ts"),
    ui_template!("src/mainview/hooks/use-mobile.ts"),
    ui_template!("src/mainview/index.css"),
    ui_template!("src/mainview/index.html"),
    ui_template!("src/mainview/lib/Layout.tsx"),
    ui_template!("src/mainview/lib/utils.ts"),
    ui_template!("src/mainview/main.tsx"),
    ui_template!("src/serial-test.ts"),
    ui_template!("src/serial-worker.ts"),
    ui_template!("src/shared/Protocol/ModuleCommand.ts"),
    ui_template!("src/shared/Protocol/ModuleDefinitionSchema.ts"),
    ui_template!("src/shared/Protocol/ModuleEven.ts"),
    ui_template!("src/shared/rpc.ts"),
    ui_template!("src/types/bun-serialport.d.ts"),
    ui_template!("tsconfig.json"),
    ui_template!("vite.config.ts"),
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



