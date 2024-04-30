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

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let rpc_url = "https://starknet-mainnet.public.blastapi.io";
    let starknet_client =
        Arc::new(StarknetClientHttp::new(&rpc_url).expect("Failed to create Starknet client"));

    let config = PontosConfig {
        indexer_version: String::from("0.0.1"),
        indexer_identifier: "task_1234".to_string(),
    };

    let pontos = Arc::new(Pontos::new(
        Arc::clone(&starknet_client),
        Arc::new(DefaultStorage::new()),
        Arc::new(DefaultEventHandler::new()),
        config,
    ));

    pontos
        .index_block_range(BlockId::Number(1000), BlockId::Number(1010), false)
        .await?;

    Ok(())
}
