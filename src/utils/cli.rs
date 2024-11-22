use colored::Colorize;
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
        #[arg(short, long, default_value_t = false)]
        local: bool,
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

                println!(
                    "{} {}",
                    format!(" SUCCESS ").white().on_green(),
                    format!(
                        " {} registry updated!",
                        if local { "Local" } else { "Global" }
                    ).green()
                );
            } else {
                println!(
                    "{} {}",
                    format!(" ERROR ").white().on_red(),
                    "Registry not found!".red()
                );
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
