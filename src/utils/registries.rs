use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::env;
use std::{collections::HashMap, path::Path};
use tokio::fs::{read_to_string, write};
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Registry {
    pub registry: String,
    pub home: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Store {
    // This will allow TOML to map to a HashMap of `String` -> `Registry`
    #[serde(flatten)]
    pub registries: HashMap<String, Registry>,
    pub current_use: Option<String>,
}

impl Store {
    pub async fn load() -> Store {
        let file_path = "registries.toml";

        if let Ok(current_dir) = env::current_dir() {
            let absolute_path: std::path::PathBuf = current_dir.join(file_path);
            println!(
                "{} {}",
                format!("Absolute path of registries list").blue().bold(),
                format!("{}", absolute_path.display())
            );
        }

        if Path::new(file_path).exists() {
            let content = read_to_string(file_path).await.unwrap_or_default();
            let value: Store = toml::from_str(&content).unwrap_or_default();
            value
        } else {
            Store {
                registries: HashMap::new(),
                current_use: Some("hello".to_string()),
            }
        }
    }

    pub async fn save(&self) {
        let content = toml::to_string_pretty(self).unwrap();
        write("registries.toml", content).await.unwrap();
    }

    pub fn list_registries(&self) {
        println!("{}", "Registry List:".bold().cyan());
        println!("{:?}", &self);

        for (name, registry) in self.registries.iter() {
            if let Some(current) = &self.current_use {
                if name == current {
                    println!(
                        "{} -> {} [{}]",
                        name.green().bold(),
                        registry.registry.yellow(),
                        "CURRENT".white().on_green()
                    );
                } else {
                    println!("{} -> {}", name.green(), registry.registry.yellow());
                }
            } else {
                println!("{} -> {}", name.green(), registry.registry.yellow());
            }
        }
    }

    pub fn set_current_use(&mut self, name: &str) {
        if self.registries.contains_key(name) {
            self.current_use = Some(name.to_string());
            println!("{} {}", "Switched to registry:".cyan(), name.green().bold());
        } else {
            println!("{} {}", "Registry not found:".red(), name.red().bold());
        }
    }
}
