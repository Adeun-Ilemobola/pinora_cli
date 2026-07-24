use crate::global_definition::Template;

macro_rules! root_folder {
    ($path:literal) => {
        Template {
            name: $path,
            source_path: concat!(
                "https://raw.githubusercontent.com/Adeun-Ilemobola/Pinora_Templat/main/",
                $path
            ),
            output_path: $path,
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

pub(crate) static ROOT_TEMPLATE_LIST: [Template; 2] = [root_folder!("justfile") , root_folder!("gitignore")];
