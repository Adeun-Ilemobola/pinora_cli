use crate::{
    global_definition::{ SourceTemplate},
   
};

macro_rules! ui_template {
    ($path:literal) => {
        SourceTemplate {
            name: $path,
            github_source_url: concat!(
                "https://raw.githubusercontent.com/Adeun-Ilemobola/Pinora_Templat/migration/slint-ui-protocol-workspace/UI/",
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
                "https://raw.githubusercontent.com/Adeun-Ilemobola/Pinora_Templat/migration/slint-ui-protocol-workspace/UI/",
                $path
            ),
            output_path: $path,
            edits: &[
                $( $edit ),+
            ],
        }
    };
}

pub static UI_TEMPLATE_LIST: [SourceTemplate; 64] = [
    ui_template!(".cargo/config.toml"),
    ui_template!(".vscode/extensions.json"),
    ui_template!(".vscode/settings.json"),
    ui_template!("Cargo.toml"),
    ui_template!("LICENSE"),
    ui_template!("README.md"),
    ui_template!("build.rs"),
    ui_template!("src/main.rs"),
    ui_template!("src/module_controller/mod.rs"),
    ui_template!("src/module_controller/module_definition.rs"),
    ui_template!("src/module_controller/module_methods/button.rs"),
    ui_template!("src/module_controller/module_methods/imu.rs"),
    ui_template!("src/module_controller/module_methods/joystick.rs"),
    ui_template!("src/module_controller/module_methods/led.rs"),
    ui_template!("src/module_controller/module_methods/led_cluster.rs"),
    ui_template!("src/module_controller/module_methods/lidar.rs"),
    ui_template!("src/module_controller/module_methods/mod.rs"),
    ui_template!("src/module_controller/module_methods/rangefinder.rs"),
    ui_template!("src/module_controller/module_methods/remote_receiver.rs"),
    ui_template!("src/module_controller/module_methods/rfid.rs"),
    ui_template!("src/module_controller/module_methods/servo.rs"),
    ui_template!("src/module_controller/module_methods/shared.rs"),
    ui_template!("src/module_controller/module_methods/stepper_motor.rs"),
    ui_template!("src/module_controller/module_methods/syslog.rs"),
    ui_template!("src/transport/bluetooth_transport.rs"),
    ui_template!("src/transport/mod.rs"),
    ui_template!("src/transport/serial_transport.rs"),
    ui_template!("src/transport/transport_gate.rs"),
    ui_template!("src/transport/transport_type.rs"),
    ui_template!("src/transport/wifi_transport.rs"),
    ui_template!("src/type_box.rs"),
    ui_template!("src/ui_bridge/mod.rs"),
    ui_template!("src/ui_bridge/publication.rs"),
    ui_template!("src/ui_bridge/transport_form.rs"),
    ui_template!("ui/app-window.slint"),
    ui_template!("ui/components/ModuleView/module-display.slint"),
    ui_template!("ui/components/card.slint"),
    ui_template!("ui/components/connection-card.slint"),
    ui_template!("ui/components/connection-status.slint"),
    ui_template!("ui/components/dashboard-summary.slint"),
    ui_template!("ui/components/form-controls.slint"),
    ui_template!("ui/components/system-status-card.slint"),
    ui_template!("ui/components/top-tabs.slint"),
    ui_template!("ui/components/transport-form.slint"),
    ui_template!("ui/module_definitions/button.slint"),
    ui_template!("ui/module_definitions/imu.slint"),
    ui_template!("ui/module_definitions/joystick.slint"),
    ui_template!("ui/module_definitions/led.slint"),
    ui_template!("ui/module_definitions/led_cluster.slint"),
    ui_template!("ui/module_definitions/lidar.slint"),
    ui_template!("ui/module_definitions/mod.slint"),
    ui_template!("ui/module_definitions/rangefinder.slint"),
    ui_template!("ui/module_definitions/remote_receiver.slint"),
    ui_template!("ui/module_definitions/rfid.slint"),
    ui_template!("ui/module_definitions/servo.slint"),
    ui_template!("ui/module_definitions/shared.slint"),
    ui_template!("ui/module_definitions/stepper_motor.slint"),
    ui_template!("ui/module_definitions/syslog.slint"),
    ui_template!("ui/pages/dashboard.slint"),
    ui_template!("ui/pages/modules.slint"),
    ui_template!("ui/pages/playground.slint"),
    ui_template!("ui/shared/module-updates.slint"),
    ui_template!("ui/shared/theme.slint"),
    ui_template!("ui/shared/types.slint"),
];
