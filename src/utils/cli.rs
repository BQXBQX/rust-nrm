use colored::Colorize;
use regex::Regex;
use std::env;
use std::path::Path;
use tokio::fs::{read_to_string, write};

use crate::utils::registries::Registry;
use clap::{Parser, Subcommand};

use super::{registries::Store, Logger};

#[derive(Parser, Debug)]
#[command(name = "rnrm")]
#[command(version = "1.0")]
#[command(about = "A Rust-based NPM Registry Manager 🦀")]
#[command(
    long_about = "RNRM helps you easily switch between different npm registries. It supports both global and local registry configuration."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all available registries
    #[command(about = "List all available registries")]
    #[command(
        long_about = "Display a list of all configured registries with their URLs. Currently active registries (global/local) will be highlighted."
    )]
    Ls,

    /// Switch to a different registry
    #[command(about = "Switch to a different registry")]
    #[command(
        long_about = "Change the active npm registry. Use --local flag to change only the current directory's registry."
    )]
    Use {
        /// Name of the registry to use (e.g., npm, yarn, taobao)
        #[arg(required = true, value_name = "REGISTRY")]
        registry: String,

        /// Apply changes only to the current directory
        #[arg(short, long, default_value_t = false)]
        local: bool,
    },

    /// Test registry response times
    #[command(about = "Test registry response times")]
    #[command(
        long_about = "Measure and compare response times for all configured registries to help you choose the fastest one."
    )]
    Test,

    /// Add a new registry
    #[command(about = "Add a new registry")]
    #[command(
        long_about = "Add a custom registry with its URL and optional homepage. The registry will be available for use immediately."
    )]
    Add {
        /// Name for the new registry
        #[arg(required = true, value_name = "NAME")]
        registry: String,

        /// Registry URL (e.g., https://registry.npmjs.org/)
        #[arg(required = true, value_name = "URL")]
        url: String,

        /// Homepage URL for the registry
        #[arg(value_name = "HOMEPAGE")]
        home: Option<String>,
    },

    /// Remove a registry
    #[command(about = "Remove a registry")]
    #[command(
        long_about = "Remove a registry from the configuration. Built-in registries cannot be removed."
    )]
    Remove {
        /// Name of the registry to remove
        #[arg(required = true, value_name = "REGISTRY")]
        registry: String,
    },
}

pub async fn execute_command(command: Commands, store: &mut Store) {
    match command {
        // list command
        Commands::Ls => {
            store.list_registries().await;
        }

        // use command
        Commands::Use { registry, local } => {
            if let Some(registry_data) = store.registries.get(&registry) {
                let registry_text = format!("registry={}", registry_data.registry);

                let npmrc_path = if local {
                    ".npmrc".to_string()
                } else {
                    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
                    format!("{}/.npmrc", home_dir)
                };

                if let Ok(current_dir) = env::current_dir() {
                    let absolute_path = if local {
                        current_dir.join(&npmrc_path)
                    } else {
                        Path::new(&npmrc_path).to_path_buf()
                    };
                    println!(
                        "{} {}",
                        format!("Absolute path of .npmrc:").blue().bold(),
                        format!("{}", absolute_path.display())
                    );
                }

                if Path::new(&npmrc_path).exists() {
                    let content = read_to_string(&npmrc_path).await.unwrap();
                    let re = Regex::new(r"(?m)^\s*registry\s*=\s*.*$").unwrap();
                    let updated_content = re.replace_all(&content, &registry_text).to_string();

                    write(&npmrc_path, updated_content).await.unwrap();
                } else {
                    write(&npmrc_path, registry_text).await.unwrap();
                }

                // set current use registry
                // store.set_current_use(&registry, local);
                store.save().await;

                Logger::success(&format!(
                    "{} registry updated!",
                    if local { "Local" } else { "Global" }
                ));
            } else {
                Logger::error("Registry not found!");
            }
        }

        // test command
        Commands::Test => {
            store.test_registry_speed().await;
        }

        // add command
        Commands::Add {
            registry,
            url,
            home,
        } => {
            store.registries.insert(
                registry.clone(),
                Registry {
                    registry: url.clone(),
                    home,
                },
            );

            store.save().await;
            Logger::success(&format!(
                "Registry {} added with URL: {}",
                registry.green().bold(),
                url.yellow()
            ));
        }

        // remove command
        Commands::Remove { registry } => {
            if let Some(removed) = store.registries.remove(&registry) {
                store.save().await;
                Logger::success(&format!(
                    "Registry {} removed (URL: {})",
                    registry.green().bold(),
                    removed.registry.yellow()
                ));
            } else {
                Logger::error(&format!(
                    "Registry {} not found",
                    registry.red().bold()
                ));
            }
        }
    }
}
