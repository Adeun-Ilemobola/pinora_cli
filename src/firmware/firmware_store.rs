use crate::global_definition::{CargoDependency, Template};

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

pub static FIRMWARE_DEPENDENCY_LIST: [CargoDependency; 11] = [
    CargoDependency {
        name: "anyhow",
        version: "1.0.104",
        features: &[],
    },
    CargoDependency {
        name: "serde",
        version: "1.0.229",
        features: &["derive"],
    },
    CargoDependency {
        name: "serde_json",
        version: "1.0.150",
        features: &[],
    },
    CargoDependency {
        name: "log",
        version: "0.4.33",
        features: &[],
    },
    CargoDependency {
        name: "pwm-pca9685",
        version: "1.0.0",
        features: &[],
    },
    CargoDependency {
        name: "uuid",
        version: "1.24.0",
        features: &["v4"],
    },
    CargoDependency {
        name: "esp-idf-svc",
        version: "0.52.1",
        features: &["critical-section", "embassy-time-driver", "embassy-sync"],
    },
    CargoDependency {
        name: "embedded-hal-bus",
        version: "0.3",
        features: &["std"],
    },
    CargoDependency {
        name: "embedded-hal-compat",
        version: "0.13",
        features: &[],
    },
    CargoDependency {
        name: "embassy-time",
        version: "0.5",
        features: &["generic-queue-8"],
    },
    CargoDependency {
        name: "vl53l1x-uld",
        version: "2.0.1",
        features: &[],
    },
];

pub static FIRMWARE_TEMPLATE_LIST: [Template; 15] = [
    // Core
    firmware_template!("src/core/hardware.rs"),
    firmware_template!("src/core/mod.rs"),
    firmware_template!("src/core/modulecore.rs"),

    // Modules
    firmware_template!("src/module/buttonmodule.rs"),
    firmware_template!("src/module/ledmodule.rs"),
    // firmware_template!("src/module/lidar.rs"),
    firmware_template!("src/module/mod.rs"),
    // firmware_template!("src/module/range_finder.rs"),
    // firmware_template!("src/module/servomodule.rs"),

    // Protocol
    firmware_template!("src/protocol/command.rs"),
    firmware_template!("src/protocol/global_definitions.rs"),
    firmware_template!("src/protocol/mod.rs"),
    firmware_template!("src/protocol/module_event.rs"),
    firmware_template!("src/protocol/registration.rs"),

    // Utilities
    firmware_template!("src/utilities/logger.rs"),
    firmware_template!("src/utilities/math.rs"),
    firmware_template!("src/utilities/mod.rs"),

    // Firmware entry point
    firmware_template!("src/main.rs"),
];