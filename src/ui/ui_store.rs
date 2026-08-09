use crate::{
    global_definition::{ SourceTemplate},
   
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

