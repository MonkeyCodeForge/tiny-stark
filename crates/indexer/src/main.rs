mod event_handler;
mod storage;

use anyhow::Result;
use dotenv::dotenv;
use event_handler::DefaultEventHandler;
use pontos::{Pontos, PontosConfig};
use starknet::core::types::BlockId;
use std::sync::Arc;
use storage::DefaultStorage;
use tiny_starknet::client::{StarknetClient, StarknetClientHttp};
use tracing::{span, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    init_tracing();

    let starknet_client = Arc::new(
        StarknetClientHttp::new(&"https://starknet-mainnet.public.blastapi.io")
            .expect("Failed to create Starknet client"),
    );

    let pontos = Arc::new(Pontos::new(
        Arc::clone(&starknet_client),
        Arc::new(DefaultStorage::new()),
        Arc::new(DefaultEventHandler::new()),
        PontosConfig {
            indexer_version: String::from("0.1.0"),
            indexer_identifier: "main".to_string(),
        },
    ));

    pontos
        .index_block_range(BlockId::Number(556049), BlockId::Number(556049), false)
        .await?;

    Ok(())
}

fn init_tracing() {
    // Initialize the LogTracer to convert `log` records to `tracing` events
    tracing_log::LogTracer::init().expect("Setting log tracer failed.");

    // Create the layers
    let env_filter = EnvFilter::from_default_env();
    let fmt_layer = fmt::layer();

    // Combine layers and set as global default
    let subscriber = Registry::default().with(env_filter).with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed.");

    let main_span = span!(Level::TRACE, "main");
    let _main_guard = main_span.enter();
}
