use clap::Parser;
use rnrm::utils::{
    cli::{execute_command, Cli},
    registries::Store,
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut store = Store::load().await;

    execute_command(cli.command, &mut store).await;
}
