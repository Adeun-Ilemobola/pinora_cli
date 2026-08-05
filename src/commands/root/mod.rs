use crate::global_definition::{SourceTemplate, Template, TemplateEdit, TemplateValue};

// macro_rules! root_folder {
//     ($path:literal) => {
//         Template {
//             name: $path,
//             source_path: concat!(
//                 "https://raw.githubusercontent.com/Adeun-Ilemobola/Pinora_Templat/main/",
//                 $path
//             ),
//             output_path: $path,
//         }
//     };
// }

macro_rules! root_item {
    ($path:literal) => {
        SourceTemplate {
            name: $path,
            github_source_url: concat!(
                "https://raw.githubusercontent.com/Adeun-Ilemobola/Pinora_Templat/feat/electrobun-architecture/",
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
                "https://raw.githubusercontent.com/Adeun-Ilemobola/Pinora_Templat/feat/electrobun-architecture/",
                $path
            ),
            output_path: $path,
            edits: &[
                $( $edit ),+
            ],
        }
    };
}

macro_rules! root_toml_file {
    ($name:expr, $firmware_path:expr, $ui_path:expr) => {
        format!(
            r#"schema_version = 1

[project]
name = "{}"
version = "0.1.0"

[paths]
firmware = "{}"
ui = "{}"
config = ".espConfig"

[device]
board = "esp32s3"
port = "auto"
baud_rate = 115200
            
"#,
            $name, $firmware_path, $ui_path
        )
    };
}

pub(crate) use root_toml_file;

// pub(crate) static ROOT_TEMPLATE_LIST: [Template; 12] = [
//     root_folder!(".gitignore"),
//     root_folder!("Cargo.lock"),
//     root_folder!("Cargo.toml"),
//     root_folder!("README.md"),
//     root_folder!("justfile"),
//     root_folder!("lib.rs"),
//     root_folder!("protocol/command.rs"),
//     root_folder!("protocol/global_definitions.rs"),
//     root_folder!("protocol/mod.rs"),
//     root_folder!("protocol/module_event.rs"),
//     root_folder!("protocol/registration.rs"),
//     root_folder!("tests/protocol_wire.rs"),
// ];

pub(crate) static NEW_ROOT_TEMPLATE_LIST: [SourceTemplate; 3] = [
    root_item!("justfile"),
    root_item!(".gitignore"),
    root_item!(
        "pinora.toml",
        TemplateEdit::InsertAfter {
            target: "Pinora_Template",
            content: TemplateValue::ProjectName,
            new_line:false
        },
    ),
];
