use std::{env, path::Path};

use clap::{Parser, Subcommand};
use regex::Regex;
use tokio::fs::{read_to_string, write};

use crate::utils::registries::Registry;

use super::registries::Store;

#[derive(Parser, Debug)]
#[command(name = "rnrm")]
#[command(version = "1.0")]
#[command(about = "manage npm registries base rust 🦀")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // List all registries
    Ls,

    // Change registry
    Use {
        #[arg(required = true)]
        registry: String,
        // #[arg(short, long)]
        // local: bool,
    },

    // Test response time for all registries
    Test,

    // Add a custom registry
    Add {
        registry: String,
        url: String,
        #[arg(default_value = "")]
        home: Option<String>,
    },

    // remove a custom registry
    Remove {
        registry: String,
    },
}

pub async fn execute_command(command: Commands, store: &mut Store) {
    match command {
        // list command
        Commands::Ls => {
            store.list_registries();
        }

        // use command
        Commands::Use { registry } => {
            if let Some(registry_data) = store.registries.get(&registry) {
                let registry_text = format!("registry={}", registry_data.registry);
                let npmrc_path = ".npmrc";

                if let Ok(current_dir) = env::current_dir() {
                    let absolute_path: std::path::PathBuf = current_dir.join(npmrc_path);
                    // println!("Absolute path of .npmrc: {} \n", absolute_path.display());
                }

                if Path::new(npmrc_path).exists() {
                    let content = read_to_string(npmrc_path).await.unwrap();
                    let re = Regex::new(r"(?m)^\s*registry\s*=\s*.*$").unwrap();
                    let updated_content = re.replace_all(&content, &registry_text).to_string();

                    write(npmrc_path, updated_content).await.unwrap();
                } else {
                    write(npmrc_path, registry_text).await.unwrap();
                }
                // println!("Current Dir registry updated!");
            } else {
                eprintln!("Registry not found!")
            }
        }

        // test command
        Commands::Test => {
            eprintln!("is developing")
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
                    registry: url,
                    home,
                },
            );

            store.save().await;
            // println!("Added registry: {}", registry);
        }

        // remove command
        Commands::Remove { registry } => {
            store.registries.remove(&registry);
            store.save().await;
            // println!("Removed registry: {}", registry);
        }
    }
}
