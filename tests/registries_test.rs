use rust_nrm::utils::registries::{Store, Registry};
use tokio::fs;

async fn setup() -> Store {
    let store = Store::load().await;
    store
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
async fn test_load_default_config() {
    cleanup().await;
    let store = Store::load().await;
    
    // Verify default registries exist
    assert!(store.registries.contains_key("npm"));
    assert!(store.registries.contains_key("yarn"));
    assert!(store.registries.contains_key("taobao"));
    assert!(store.registries.contains_key("tencent"));

    // Verify npm registry URL
    let npm_registry = store.registries.get("npm").unwrap();
    assert_eq!(npm_registry.registry, "https://registry.npmjs.org/");
    
    cleanup().await;
}

#[tokio::test]
async fn test_add_registry() {
    let mut store = setup().await;
    
    // Add a new registry
    let name = "test-registry";
    let url = "https://test.registry.com";
    let home = Some("https://test.com".to_string());
    
    store.registries.insert(name.to_string(), Registry {
        registry: url.to_string(),
        home,
    });

    // Verify the registry was added
    let added_registry = store.registries.get(name).unwrap();
    assert_eq!(added_registry.registry, url);
    assert_eq!(added_registry.home, Some("https://test.com".to_string()));
    
    cleanup().await;
}

#[tokio::test]
async fn test_remove_registry() {
    let mut store = setup().await;
    
    // Add and then remove a registry
    let name = "test-registry";
    store.registries.insert(name.to_string(), Registry {
        registry: "https://test.com".to_string(),
        home: None,
    });
    
    assert!(store.registries.contains_key(name));
    store.registries.remove(name);
    assert!(!store.registries.contains_key(name));
    
    cleanup().await;
}

#[tokio::test]
async fn test_save_and_load() {
    cleanup().await;  // Start with a clean state
    let mut store = setup().await;
    
    // Add a test registry
    store.registries.insert("test".to_string(), Registry {
        registry: "https://test.com".to_string(),
        home: Some("https://test.com".to_string()),
    });
    
    // Save the store
    store.save().await;
    
    // Ensure the file exists before trying to load it
    let config_path = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config")
        .join("rust-nrm")
        .join("registries.toml");
    assert!(config_path.exists(), "Config file should exist after save");
    
    // Load a new store and verify the data
    let loaded_store = Store::load().await;
    assert!(loaded_store.registries.contains_key("test"), "Test registry should exist in loaded store");
    let test_registry = loaded_store.registries.get("test").unwrap();
    assert_eq!(test_registry.registry, "https://test.com", "Registry URL should match");
    assert_eq!(test_registry.home, Some("https://test.com".to_string()), "Home URL should match");
    
    cleanup().await;
}
