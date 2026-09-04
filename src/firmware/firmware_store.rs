use crate::global_definition::{SourceTemplate};

macro_rules! firmware_template {
    ($path:literal) => {
        SourceTemplate {
            name: $path,
            github_source_url: concat!(
                "https://raw.githubusercontent.com/Adeun-Ilemobola/Pinora_Templat/migration/slint-ui-protocol-workspace/Firmware_Templates/",
                $path
            ),
            output_path: $path,
            edits: &[],
        }
    };
}


pub static FIRMWARE_TEMPLATE_LIST: [SourceTemplate; 26] = [
    firmware_template!(".cargo/config.toml"),
    firmware_template!("Cargo.toml"),
    firmware_template!("build.rs"),
    firmware_template!("pre-script.rhai"),
    firmware_template!("rust-toolchain.toml"),
    firmware_template!("sdkconfig.defaults"),
    firmware_template!("src/core/emitter.rs"),
    firmware_template!("src/core/hardware.rs"),
    firmware_template!("src/core/mod.rs"),
    firmware_template!("src/core/modulecore.rs"),
    firmware_template!("src/main.rs"),
    firmware_template!("src/module/buttonmodule.rs"),
    firmware_template!("src/module/imu/imu_type.rs"),
    firmware_template!("src/module/imu/mod.rs"),
    firmware_template!("src/module/imu/mpu_impl.rs"),
    firmware_template!("src/module/joystick.rs"),
    firmware_template!("src/module/ledmodule.rs"),
    firmware_template!("src/module/lidar.rs"),
    firmware_template!("src/module/mod.rs"),
    firmware_template!("src/module/range_finder.rs"),
    firmware_template!("src/module/remote_receiver.rs"),
    firmware_template!("src/module/rfid.rs"),
    firmware_template!("src/module/servomodule.rs"),
    firmware_template!("src/module/stepper.rs"),
    firmware_template!("src/utilities/math.rs"),
    firmware_template!("src/utilities/mod.rs"),
];
