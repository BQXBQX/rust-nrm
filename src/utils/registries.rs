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

    // current work dir use registry
    pub current_use_local: Option<String>,

    // current global use registry
    pub current_use_global: Option<String>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            registries: HashMap::new(),
            current_use_local: None,
            current_use_global: None,
        }
    }

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
                current_use_local: None,
                current_use_global: None,
            }
        }
    }

    pub async fn save(&self) {
        let content = toml::to_string_pretty(self).unwrap();
        write("registries.toml", content).await.unwrap();
    }

    pub fn list_registries(&self) {
        println!("Available registries:");

        for (name, registry) in self.registries.iter() {
            let mut tags = Vec::new();

            if let Some(current_global) = &self.current_use_global {
                if name == current_global {
                    tags.push("[GLOBAL]".white().on_blue());
                }
            }

            if let Some(current_local) = &self.current_use_local {
                if name == current_local {
                    tags.push("[LOCAL]".white().on_green());
                }
            }

            let tags_str = if !tags.is_empty() {
                format!(
                    " {}",
                    tags.iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            } else {
                String::new()
            };

            println!(
                "{} -> {}{}",
                name.green().bold(),
                registry.registry.yellow(),
                tags_str
            );
        }
    }

    pub fn set_current_use(&mut self, name: &str, is_local: bool) {
        if self.registries.contains_key(name) {
            if is_local {
                self.current_use_local = Some(name.to_string());
                println!(
                    "{} {} ({})",
                    "Switched to registry:".cyan(),
                    name.green().bold(),
                    "local".yellow()
                );
            } else {
                self.current_use_global = Some(name.to_string());
                println!(
                    "{} {} ({})",
                    "Switched to registry:".cyan(),
                    name.green().bold(),
                    "global".yellow()
                );
            }
        } else {
            println!("{} {}", "Registry not found:".red(), name.red().bold());
        }
    }
}
