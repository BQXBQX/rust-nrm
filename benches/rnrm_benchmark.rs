use criterion::{criterion_group, criterion_main, Criterion};
use rnrm::utils::{
    cli::{CommandExecutor, Commands},
    registries::{Registry, Store},
};
use tokio::runtime::Runtime;

fn bench_ls(c: &mut Criterion) {
    c.bench_function("cli_ls", |b| {
        b.iter(|| {
            // Create a mock store
            let mut store = Store {
                registries: std::collections::HashMap::new(),
            };

            // Add a sample registry to the store
            store.registries.insert(
                "example_registry".to_string(),
                Registry {
                    registry: "https://example.com".to_string(),
                    home: Some("https://example.com".to_string()),
                },
            );

            // Create a runtime for async operations
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let mut executor = CommandExecutor::new(store);
                executor.execute(Commands::Ls).await;
            });
        })
    });
}

fn bench_use(c: &mut Criterion) {
    c.bench_function("cli_use", |b| {
        b.iter(|| {
            // Create a mock store
            let mut store = Store {
                registries: std::collections::HashMap::new(),
            };

            // Add a sample registry to the store
            store.registries.insert(
                "example_registry".to_string(),
                Registry {
                    registry: "https://example.com".to_string(),
                    home: Some("https://example-home.com".to_string()),
                },
            );

            // Simulate calling the Use command
            let registry_name = "example_registry".to_string();
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let mut executor = CommandExecutor::new(store);
                executor.execute(Commands::Use {
                    registry: registry_name,
                    local: false,
                }).await;
            });
        });
    });
}

fn bench_add(c: &mut Criterion) {
    c.bench_function("cli_add", |b| {
        b.iter(|| {
            // Create a mock store
            let mut store = Store {
                registries: std::collections::HashMap::new(),
            };

            // Simulate calling the Add command
            let registry_name = "new_registry".to_string();
            let registry_url = "https://new-url.com".to_string();
            let home = Some("https://home.com".to_string());

            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let mut executor = CommandExecutor::new(store);
                executor.execute(Commands::Add {
                    registry: registry_name,
                    url: registry_url,
                    home,
                }).await;
            });
        });
    });
}

fn bench_remove(c: &mut Criterion) {
    c.bench_function("cli_remove", |b| {
        b.iter(|| {
            // Create a mock store
            let mut store = Store {
                registries: std::collections::HashMap::new(),
            };

            // Add a sample registry to remove
            store.registries.insert(
                "example_registry".to_string(),
                Registry {
                    registry: "https://example.com".to_string(),
                    home: Some("https://example-home.com".to_string()),
                },
            );

            // Simulate calling the Remove command
            let registry_name = "example_registry".to_string();
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let mut executor = CommandExecutor::new(store);
                executor.execute(Commands::Remove {
                    registry: registry_name,
                }).await;
            });
        });
    });
}

criterion_group!(benches, bench_ls, bench_use, bench_add, bench_remove);
criterion_main!(benches);
