use crate::{
    firmware::firmware_store::firmware_template, global_definition::SourceTemplate, project_config::load_config, ui::ui_store::ui_template, utility::generate_file,
};
use std::path::PathBuf;

pub struct Component {
    pub name: String,
    pub firmware_file: Vec<SourceTemplate>,
    pub ui_file: Vec<SourceTemplate>,
}

pub struct ComponentCore {
    pub components: Vec<Component>,
    root_dir: PathBuf,
}
    
impl ComponentCore {
    pub fn new() -> Result<Self, String> {
        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(error) => {
                return Err(error.to_string());
            }
        };

        let led = Component {
            name: "LED".to_string(),
            firmware_file: vec![firmware_template!("src/module/ledmodule.rs")],
            ui_file: vec![ui_template!("src/lib/Modules/Led.tsx")],
        };

        Ok(Self {
            components: vec![led],
            root_dir: current_dir,
        })
    }

    fn components_valid(&self, name: &str) -> Option<&Component> {
        // Implement validation logic for the components here
        for component in self.components.iter() {
            if component.name == name {
                return Some(component);
            }
        }
        None
    }

     pub async  fn add_component(&mut self, component: &str) {
        let config;
        match load_config() {
           Some(cfg) => {
               config = cfg;
               println!("Loaded config successfully for component {}", config);
           }
           None => {
               eprintln!("Failed to load config");
               return;
           }
        }
        let ui_root = PathBuf::from(config.ui_path.clone());
        let firmware_root = PathBuf::from(config.firmware_path.clone());

        if (!firmware_root.exists() || !ui_root.exists()) || firmware_root.is_file() || ui_root.is_file() {
            eprintln!("Firmware or UI root does not exist or is not a directory");
            return;
        }
        if let Some(comp) = self.components_valid(component) {

            for ff in &comp.firmware_file {
               match generate_file(ff, &firmware_root, &config).await {
                   Ok(_) => {
                       println!("
                       Generated firmware file for 
                       component {} | 
                       root_dir: {} |
                       firmware_root: {} |
                       ui_root: {} |
                       source_file: {} |
                       
                       ", component, self.root_dir.display(), firmware_root.display(), ui_root.display(), ff);
                   },
                   Err(error) => {
                       eprintln!("Failed to generate firmware file: {}", error);
                   }
               }
            }
            for uf in &comp.ui_file {
                match generate_file(uf, &ui_root, &config).await {
                    Ok(_) => {
                         println!("
                       Generated UI file for 
                       component {} | 
                       root_dir: {} |
                       firmware_root: {} |
                       ui_root: {} |
                       source_file: {} |
                       
                       ", component, self.root_dir.display(), firmware_root.display(), ui_root.display(), uf);
                        
                    },
                    Err(error) => {
                        eprintln!("Failed to generate UI file: {}", error);
                    }
                };
            }
        }else {
            eprintln!("Component '{}' not found", component);
        }
    }

    pub  fn list_all_components(&self)  -> Vec<String> {

        self.components.iter().map(|c| c.name.clone()).collect()
    }
}
