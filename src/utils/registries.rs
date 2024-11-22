use colored::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use toml;

use super::speed_test::{SpeedTestResult, SpeedTester};
use super::Logger;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Registry {
    pub registry: String,
    pub home: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Store {
    pub registries: HashMap<String, Registry>,
}

impl Store {
    pub async fn load() -> Self {
        let config_path = get_config_path();
        Logger::info(&format!("Config file path: {}", config_path.display()));

        if !config_path.exists() {
            Logger::info("Config file not found, creating default configuration...");
            Self::create_default_config(&config_path).await;
        }

        match fs::read_to_string(&config_path).await {
            Ok(contents) => {
                match toml::from_str(&contents) {
                    Ok(registries) => Self { registries },
                    Err(e) => {
                        Logger::error(&format!("Failed to parse config file: {}", e));
                        Self::create_default_store()
                    }
                }
            }
            Err(e) => {
                Logger::error(&format!("Failed to read config file: {}", e));
                Self::create_default_store()
            }
        }
    }

    async fn create_default_config(config_path: &Path) {
        let default_registries = HashMap::from([
            (
                "npm".to_string(),
                Registry {
                    registry: "https://registry.npmjs.org/".to_string(),
                    home: Some("https://www.npmjs.org".to_string()),
                },
            ),
            (
                "yarn".to_string(),
                Registry {
                    registry: "https://registry.yarnpkg.com/".to_string(),
                    home: Some("https://yarnpkg.com".to_string()),
                },
            ),
            (
                "taobao".to_string(),
                Registry {
                    registry: "https://registry.npmmirror.com/".to_string(),
                    home: Some("https://npmmirror.com/".to_string()),
                },
            ),
            (
                "tencent".to_string(),
                Registry {
                    registry: "https://mirrors.cloud.tencent.com/npm/".to_string(),
                    home: Some("https://mirrors.cloud.tencent.com/npm/".to_string()),
                },
            ),
        ]);

        if let Some(parent) = config_path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent).await {
                    Logger::error(&format!("Failed to create config directory: {}", e));
                    return;
                }
            }
        }

        let toml = toml::to_string(&default_registries).unwrap();
        if let Err(e) = fs::write(config_path, toml).await {
            Logger::error(&format!("Failed to write default config: {}", e));
        }
    }

    fn create_default_store() -> Self {
        Self {
            registries: HashMap::from([
                (
                    "npm".to_string(),
                    Registry {
                        registry: "https://registry.npmjs.org/".to_string(),
                        home: Some("https://www.npmjs.org".to_string()),
                    },
                ),
                (
                    "yarn".to_string(),
                    Registry {
                        registry: "https://registry.yarnpkg.com/".to_string(),
                        home: Some("https://yarnpkg.com".to_string()),
                    },
                ),
                (
                    "taobao".to_string(),
                    Registry {
                        registry: "https://registry.npmmirror.com/".to_string(),
                        home: Some("https://npmmirror.com/".to_string()),
                    },
                ),
                (
                    "tencent".to_string(),
                    Registry {
                        registry: "https://mirrors.cloud.tencent.com/npm/".to_string(),
                        home: Some("https://mirrors.cloud.tencent.com/npm/".to_string()),
                    },
                ),
            ])
        }
    }

    pub async fn save(&self) {
        let config_path = get_config_path();
        let content = toml::to_string_pretty(&self.registries).unwrap();
        if let Err(e) = fs::write(&config_path, content).await {
            Logger::error(&format!("Failed to save config: {}", e));
        }
    }

    pub async fn get_current_registry(&self, is_local: bool) -> Option<String> {
        let npmrc_path = if is_local {
            ".npmrc".to_string()
        } else {
            dirs::home_dir()
                .map(|path| path.join(".npmrc").to_string_lossy().to_string())
                .expect("Failed to get home directory")
        };

        if Path::new(&npmrc_path).exists() {
            if let Ok(content) = fs::read_to_string(&npmrc_path).await {
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
                if is_local {
                    "local".yellow()
                } else {
                    "global".yellow()
                }
            ));
        } else {
            Logger::error(&format!("Registry not found: {}", name));
        }
    }

    pub async fn test_registry_speed(&self) -> Vec<SpeedTestResult> {
        let tester = SpeedTester::new();
        let registries: Vec<(String, String)> = self
            .registries
            .iter()
            .map(|(name, reg)| (name.clone(), reg.registry.clone()))
            .collect();

        tester.test_all(&registries).await
    }
}

fn get_config_path() -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");
    home.join(".config")
        .join("rust-nrm")
        .join("registries.toml")
}
