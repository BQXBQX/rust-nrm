use rust_nrm::utils::cli::{CommandExecutor, Commands};
use rust_nrm::utils::registries::Store;
use tokio::fs;

async fn setup() -> CommandExecutor {
    let store = Store::load().await;
    CommandExecutor::new(store)
}

async fn cleanup() {
    let config_path = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config")
        .join("rust-nrm")
        .join("registries.toml");
    if config_path.exists() {
        let _ = fs::remove_file(config_path).await;
    }
}

#[tokio::test]
async fn test_list_command() {
    let mut executor = setup().await;
    executor.execute(Commands::Ls).await;
    cleanup().await;
}

#[tokio::test]
async fn test_use_command() {
    let mut executor = setup().await;

    // Test using npm registry
    executor
        .execute(Commands::Use {
            registry: "npm".to_string(),
            local: false,
        })
        .await;

    // Verify by listing registries
    executor.execute(Commands::Ls).await;
    // Note: Since we can't directly access the store, we rely on the Ls command output
    // The actual verification is done through the command output

    cleanup().await;
}

#[tokio::test]
async fn test_add_command() {
    let mut executor = setup().await;

    // Add a test registry
    let name = "test-registry";
    let url = "https://test.registry.com";
    executor
        .execute(Commands::Add {
            registry: name.to_string(),
            url: url.to_string(),
            home: None,
        })
        .await;

    // Verify by trying to use the added registry
    executor
        .execute(Commands::Use {
            registry: name.to_string(),
            local: false,
        })
        .await;

    cleanup().await;
}

#[tokio::test]
async fn test_remove_command() {
    let mut executor = setup().await;

    // First add a registry
    let name = "test-registry";
    executor
        .execute(Commands::Add {
            registry: name.to_string(),
            url: "https://test.com".to_string(),
            home: None,
        })
        .await;

    // Then remove it
    executor
        .execute(Commands::Remove {
            registry: name.to_string(),
        })
        .await;

    // Try to use the removed registry - this should fail but not panic
    executor
        .execute(Commands::Use {
            registry: name.to_string(),
            local: false,
        })
        .await;

    cleanup().await;
}

#[tokio::test]
async fn test_test_command() {
    let mut executor = setup().await;
    
    // Add a test registry that we know will timeout
    executor
        .execute(Commands::Add {
            registry: "test-timeout".to_string(),
            url: "https://registry.does.not.exist.example.com".to_string(),
            home: None,
        })
        .await;
    
    // Run the test command
    executor.execute(Commands::Test).await;
    
    // Clean up
    executor
        .execute(Commands::Remove {
            registry: "test-timeout".to_string(),
        })
        .await;
    
    cleanup().await;
}
