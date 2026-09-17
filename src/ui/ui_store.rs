use crate::global_definition::SourceTemplate;

macro_rules! ui_template {
    ($path:literal) => {
        SourceTemplate {
            name: $path,
            github_source_url: concat!(
                "https://raw.githubusercontent.com/Adeun-Ilemobola/Pinora_Templat/main/UI/",
                $path
            ),
            output_path: $path,
            edits: &[],
        }
    };
}

pub(crate) use ui_template;
pub const UI_TEMPLATE_COUNT: usize = 80;
pub static UI_TEMPLATE_LIST: [SourceTemplate; UI_TEMPLATE_COUNT] = [
    ui_template!(".gitignore"),
    ui_template!(".vscode/extensions.json"),
    ui_template!("README.md"),
    ui_template!("bun.lock"),
    ui_template!("components.json"),
    ui_template!("index.html"),
    ui_template!("package.json"),
    ui_template!("public/tauri.svg"),
    ui_template!("public/vite.svg"),
    ui_template!("src-tauri/.gitignore"),
    ui_template!("src-tauri/Cargo.lock"),
    ui_template!("src-tauri/Cargo.toml"),
    ui_template!("src-tauri/build.rs"),
    ui_template!("src-tauri/capabilities/default.json"),
    ui_template!("src-tauri/icons/128x128.png"),
    ui_template!("src-tauri/icons/128x128@2x.png"),
    ui_template!("src-tauri/icons/32x32.png"),
    ui_template!("src-tauri/icons/Square107x107Logo.png"),
    ui_template!("src-tauri/icons/Square142x142Logo.png"),
    ui_template!("src-tauri/icons/Square150x150Logo.png"),
    ui_template!("src-tauri/icons/Square284x284Logo.png"),
    ui_template!("src-tauri/icons/Square30x30Logo.png"),
    ui_template!("src-tauri/icons/Square310x310Logo.png"),
    ui_template!("src-tauri/icons/Square44x44Logo.png"),
    ui_template!("src-tauri/icons/Square71x71Logo.png"),
    ui_template!("src-tauri/icons/Square89x89Logo.png"),
    ui_template!("src-tauri/icons/StoreLogo.png"),
    ui_template!("src-tauri/icons/icon.icns"),
    ui_template!("src-tauri/icons/icon.ico"),
    ui_template!("src-tauri/icons/icon.png"),
    ui_template!("src-tauri/src/lib.rs"),
    ui_template!("src-tauri/src/main.rs"),
    ui_template!("src-tauri/tauri.conf.json"),
    ui_template!("src/App.tsx"),
    ui_template!("src/components/AppLayout.tsx"),
    ui_template!("src/components/Esp32StatsCard.tsx"),
    ui_template!("src/components/LogViewer.tsx"),
    ui_template!("src/components/ModuleCard.tsx"),
    ui_template!("src/components/ParentControlledState.tsx"),
    ui_template!("src/components/PivotSlider.tsx"),
    ui_template!("src/components/StepperDial.tsx"),
    ui_template!("src/components/TransportForm.tsx"),
    ui_template!("src/components/theme-provider.tsx"),
    ui_template!("src/components/ui/alert.tsx"),
    ui_template!("src/components/ui/badge.tsx"),
    ui_template!("src/components/ui/button.tsx"),
    ui_template!("src/components/ui/card.tsx"),
    ui_template!("src/components/ui/field.tsx"),
    ui_template!("src/components/ui/input.tsx"),
    ui_template!("src/components/ui/label.tsx"),
    ui_template!("src/components/ui/select.tsx"),
    ui_template!("src/components/ui/separator.tsx"),
    ui_template!("src/components/ui/sheet.tsx"),
    ui_template!("src/components/ui/sidebar.tsx"),
    ui_template!("src/components/ui/skeleton.tsx"),
    ui_template!("src/components/ui/slider.tsx"),
    ui_template!("src/components/ui/switch.tsx"),
    ui_template!("src/components/ui/tabs.tsx"),
    ui_template!("src/components/ui/tooltip.tsx"),
    ui_template!("src/hooks/use-mobile.ts"),
    ui_template!("src/index.css"),
    ui_template!("src/lib/IncomingCommand.ts"),
    ui_template!("src/lib/ModuleGeter.ts"),
    ui_template!("src/lib/Modulefront.TS"),
    // ui_template!("src/lib/Modules/Led.tsx"),
     ui_template!("src/lib/Modules/button.tsx"),
    // ui_template!("src/lib/Modules/imu.tsx"),
    //ui_template!("src/lib/Modules/lidar.tsx"),
    // ui_template!("src/lib/Modules/rangefinder.tsx"),
    // ui_template!("src/lib/Modules/remote-receiver.tsx"),
    // ui_template!("src/lib/Modules/rfid.tsx"),
    // ui_template!("src/lib/Modules/servo.tsx"),
    // ui_template!("src/lib/Modules/stepper.tsx"),
    ui_template!("src/lib/logs.ts"),
    ui_template!("src/lib/protocol/command.ts"),
    ui_template!("src/lib/protocol/event.ts"),
    ui_template!("src/lib/protocol/message.ts"),
    ui_template!("src/lib/protocol/module-type.ts"),
    ui_template!("src/lib/protocol/registration.ts"),
    ui_template!("src/lib/protocol/system.ts"),
    ui_template!("src/lib/utils.ts"),
    ui_template!("src/main.tsx"),
    ui_template!("src/page/Dashboard.tsx"),
   // ui_template!("src/page/LidarPage.tsx"),
    ui_template!("src/page/LogsPage.tsx"),
    ui_template!("src/vite-env.d.ts"),
    // ui_template!("tests/frontend-review.html"),
    // ui_template!("tests/frontend-review.tsx"),
    // ui_template!("tests/lidar-dependencies.test.ts"),
    // ui_template!("tests/lidar-playground.html"),
    // ui_template!("tests/lidar-playground.tsx"),
    // ui_template!("tests/lidar.test.ts"),
    // ui_template!("tests/logs.test.ts"),
    // ui_template!("tests/parent-control.test.tsx"),
    ui_template!("tsconfig.json"),
    ui_template!("tsconfig.node.json"),
    ui_template!("vite.config.ts"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::firmware::firmware_store::FIRMWARE_TEMPLATE_LIST;
    use crate::global_definition::ProjectConfig;
    use crate::root::NEW_ROOT_TEMPLATE_LIST;
    use crate::utility::generate_file;

    #[tokio::test]
    #[ignore = "downloads the current templates from GitHub"]
    async fn scaffold_current_ui() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!("ui-scaffold-{}", uuid::Uuid::new_v4()));
        let config = ProjectConfig {
            project_name: "ui_smoke".into(),
            firmware_path: root.join("Firmware").display().to_string(),
            ui_path: root.join("UI").display().to_string(),
            id: uuid::Uuid::new_v4().to_string(),
            build_command: "just build-firmware".into(),
            flash_command: "just flash".into(),
            install_components: vec![],
        };
        for item in NEW_ROOT_TEMPLATE_LIST.iter() {
            generate_file(item, &root, &config).await.unwrap();
        }
        for item in FIRMWARE_TEMPLATE_LIST.iter() {
            generate_file(item, &root.join("Firmware"), &config).await.unwrap();
        }
        let firmware_config = std::fs::read_to_string(root.join("Firmware/.cargo/config.toml")).unwrap();
        let firmware_config = firmware_config.parse::<toml_edit::DocumentMut>().unwrap();
        assert_eq!(
            firmware_config["build"]["target-dir"].as_str().unwrap(),
            config.firmware_target_dir(cfg!(windows)).unwrap()
        );
        let target = firmware_config["build"]["target-dir"].as_str().unwrap();
        if cfg!(windows) {
            assert!(target.len() <= 10);
            assert!(target.starts_with("C:/p/"));
        }
        println!("Generated firmware target: {target}");
        assert!(root.join("Firmware/src/main.rs").is_file());
        assert!(root.join("protocol/src/lib.rs").is_file());
        for item in UI_TEMPLATE_LIST.iter() {
            generate_file(item, &root.join("UI"), &config).await.unwrap();
        }
        let icon = std::fs::read(root.join("UI/src-tauri/icons/icon.png")).unwrap();
        assert!(icon.starts_with(b"\x89PNG\r\n\x1a\n"));
        let recipes = std::fs::read_to_string(root.join("justfile")).unwrap();
        assert!(!recipes.contains("Firmware_Templates"));
        assert!(recipes.contains("cd Firmware && cargo +esp-1.93 clean"));
        assert!(recipes.contains("cd Firmware && cargo +esp-1.93 espflash flash --release --monitor"));
        assert!(!recipes.contains("cd UI && cargo build"));
        assert!(recipes.contains("bun tauri build --no-bundle"));
        let metadata = std::fs::read_to_string(root.join("pinora.toml")).unwrap();
        assert!(metadata.contains("name = \"ui_smoke\""));
        assert!(metadata.contains("firmware = \"Firmware\""));
        println!("Generated UI smoke project: {}", root.display());
    }
}
