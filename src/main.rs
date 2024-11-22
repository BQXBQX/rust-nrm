use clap::Parser;
use rust_nrm::utils::{
    cli::{Cli, CommandExecutor},
    registries::Store,
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let store = Store::load().await;
    let mut executor = CommandExecutor::new(store);
    executor.execute(cli.command).await;
}
