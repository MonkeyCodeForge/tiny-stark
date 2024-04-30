mod event_handler;
mod storage;

use anyhow::{Context, Result};
use dotenv::dotenv;
use event_handler::DefaultEventHandler;
use pontos::{Pontos, PontosConfig};
use starknet::core::types::BlockId;
use std::sync::Arc;
use storage::DefaultStorage;
use tiny_starknet::client::{StarknetClient, StarknetClientHttp};
use tracing::{info, span, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    init_tracing();

    let starknet_url = std::env::var("STARKNET_API_URL")
        .unwrap_or_else(|_| "https://starknet-mainnet.public.blastapi.io".to_string());
    info!("Using Starknet API at: {}", starknet_url);

    let starknet_client = StarknetClientHttp::new(&starknet_url)
        .context("Failed to create Starknet client")?;

    let pontos = Pontos::new(
        Arc::new(starknet_client),
        Arc::new(DefaultStorage::new()),
        Arc::new(DefaultEventHandler::new()),
        PontosConfig {
            indexer_version: String::from("0.1.0"),
            indexer_identifier: "main".to_string(),
        },
    );

    pontos
        .index_block_range(BlockId::Number(556049), BlockId::Number(556049), false)
        .await
        .context("Error indexing block range")?;

    Ok(())
}

fn init_tracing() {
    tracing_log::LogTracer::init().expect("Setting log tracer failed.");

    let subscriber = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().with_target(true));  // Ensures that the target of the event is included in the output

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed.");

    let main_span = span!(Level::TRACE, "main");
    let _main_guard = main_span.enter();
    info!("Application started");
}
