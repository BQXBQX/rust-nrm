use criterion::{criterion_group, criterion_main, Criterion};
use rnrm::utils::{
    cli::{execute_command, Commands},
    registries::{Registry, Store},
};
use tokio::runtime::Runtime;

fn bench_ls(c: &mut Criterion) {
    c.bench_function("cli_ls", |b| {
        b.iter(|| {
            // Create a mock store and call `Ls` command
            let mut store = Store {
                registries: std::collections::HashMap::new(),
                current_use_local: None,
            };

            // Add a sample registry to the store
            store.registries.insert(
                "example_registry".to_string(),
                Registry {
                    registry: "https://example.com".to_string(),
                    home: Some("https://example-home.com".to_string()),
                },
            );

            // Simulate calling the Ls command
            let rt = Runtime::new().unwrap();
            rt.block_on(execute_command(Commands::Ls, &mut store));
        });
    });
}

fn bench_use(c: &mut Criterion) {
    c.bench_function("cli_use", |b| {
        b.iter(|| {
            // Create a mock store and call `Use` command
            let mut store = Store {
                registries: std::collections::HashMap::new(),
                current_use_local: None,
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
            rt.block_on(execute_command(
                Commands::Use {
                    registry: registry_name,
                    local: false,
                },
                &mut store,
            ));
        });
    });
}

fn bench_add(c: &mut Criterion) {
    c.bench_function("cli_add", |b| {
        b.iter(|| {
            // Create a mock store and call `Add` command
            let mut store = Store {
                registries: std::collections::HashMap::new(),
                current_use_local: None,
            };

            // Simulate calling the Add command
            let registry_name = "new_registry".to_string();
            let registry_url = "https://new-url.com".to_string();
            let home = Some("https://home.com".to_string());

            let rt = Runtime::new().unwrap();
            rt.block_on(execute_command(
                Commands::Add {
                    registry: registry_name,
                    url: registry_url,
                    home,
                },
                &mut store,
            ));
        });
    });
}

fn bench_remove(c: &mut Criterion) {
    c.bench_function("cli_remove", |b| {
        b.iter(|| {
            // Create a mock store and call `Remove` command
            let mut store = Store {
                registries: std::collections::HashMap::new(),
                current_use_local: None,
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
            rt.block_on(execute_command(
                Commands::Remove {
                    registry: registry_name,
                },
                &mut store,
            ));
        });
    });
}

criterion_group!(benches, bench_ls, bench_use, bench_add, bench_remove);
criterion_main!(benches);
