use crate::global_definition::{SourceTemplate, TemplateEdit, TemplateValue};

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
    root_item!(
        "justfile",
        TemplateEdit::InsertAfter{
            target: "Firmware_Templates",
            content: TemplateValue::Literal("Firmware"),
            new_line:false
        },
        TemplateEdit::InsertAfter{
            target: "Firmware_Templates",
            content: TemplateValue::Literal("Firmware"),
            new_line:false
        },
        TemplateEdit::InsertAfter{
            target: "UI_Templates",
            content: TemplateValue::Literal("UI"),
            new_line:false
        },
        TemplateEdit::InsertAfter{
            target: "UI_Templates",
            content: TemplateValue::Literal("UI"),
            new_line:false
        }
),
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
