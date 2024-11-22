use colored::Colorize;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use std::{collections::HashMap, path::Path};
use tokio::fs::{read_to_string, write};
use toml;

use super::Logger;

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
}

impl Store {
    pub fn new() -> Store {
        Store {
            registries: HashMap::new(),
        }
    }

    pub async fn load() -> Store {
        let file_path = "registries.toml";

        if let Ok(current_dir) = env::current_dir() {
            let absolute_path: std::path::PathBuf = current_dir.join(file_path);
            Logger::info(&format!(
                "Absolute path of registries list: {}",
                absolute_path.display()
            ));
        }

        if Path::new(file_path).exists() {
            let content = read_to_string(file_path).await.unwrap_or_default();
            let value: Store = toml::from_str(&content).unwrap_or_default();
            value
        } else {
            Store {
                registries: HashMap::new(),
            }
        }
    }

    pub async fn save(&self) {
        let content = toml::to_string_pretty(self).unwrap();
        write("registries.toml", content).await.unwrap();
    }

    pub async fn get_current_registry(&self, is_local: bool) -> Option<String> {
        let npmrc_path = if is_local {
            ".npmrc".to_string()
        } else {
            let home_dir = env::var("HOME").expect("Failed to get HOME directory");
            format!("{}/.npmrc", home_dir)
        };

        if Path::new(&npmrc_path).exists() {
            if let Ok(content) = read_to_string(&npmrc_path).await {
                let re = Regex::new(r"(?m)^\s*registry\s*=\s*(.+?)\s*$").unwrap();
                if let Some(captures) = re.captures(&content) {
                    let registry_url = captures.get(1).unwrap().as_str().to_string();
                    // Find registry name by URL
                    for (name, registry) in &self.registries {
                        if registry.registry == registry_url {
                            return Some(name.clone());
                        }
                    }
                }
            }
        }
        None
    }

    pub async fn list_registries(&self) {
        Logger::list("Available registries:");

        // Get current registries
        let current_global = self.get_current_registry(false).await;
        let current_local = self.get_current_registry(true).await;

        for (name, registry) in self.registries.iter() {
            let mut tags = Vec::new();

            if let Some(current_global_name) = &current_global {
                if name == current_global_name {
                    tags.push("[GLOBAL]".white().on_blue());
                }
            }

            if let Some(current_local_name) = &current_local {
                if name == current_local_name {
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
            Logger::info(&format!(
                "Switched to registry: {} ({})",
                name.green().bold(),
                if is_local { "local".yellow() } else { "global".yellow() }
            ));
        } else {
            Logger::error(&format!("Registry not found: {}", name));
        }
    }
}
