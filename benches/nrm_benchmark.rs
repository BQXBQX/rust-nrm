use criterion::{criterion_group, criterion_main, Criterion};
use tokio::process::Command;
use tokio::runtime::Runtime;

async fn execute_nrm_command(command: &str, args: &[&str]) -> std::io::Result<()> {
    let mut cmd = Command::new("nrm");
    cmd.arg(command);
    cmd.args(args);
    cmd.output().await?;
    Ok(())
}

fn bench_ls(c: &mut Criterion) {
    c.bench_function("nrm_ls", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(execute_nrm_command("ls", &[])).unwrap();
        });
    });
}

fn bench_use(c: &mut Criterion) {
    c.bench_function("nrm_use", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(execute_nrm_command("use", &["npm"])).unwrap();
        });
    });
}

fn bench_add(c: &mut Criterion) {
    c.bench_function("nrm_add", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(execute_nrm_command("add", &["my-registry", "https://my-registry-url.com"])).unwrap();
        });
    });
}

fn bench_remove(c: &mut Criterion) {
    c.bench_function("nrm_remove", |b| {
        b.iter(|| {
            let rt = Runtime::new().unwrap();
            rt.block_on(execute_nrm_command("remove", &["my-registry"])).unwrap();
        });
    });
}

criterion_group!(benches, bench_ls, bench_use, bench_add, bench_remove);
criterion_main!(benches);
