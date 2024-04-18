use starknet::{
    core::types::{BlockId, BlockTag, FieldElement, MaybePendingBlockWithTxHashes},
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider,
    },
};
use url::Url;

#[tokio::main]
async fn main() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-mainnet.infura.io/v3/bd221cae8220402f88b090b4132679f3")
            .unwrap(),
    ));

    let latest_block = provider
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
        .await;

    match latest_block {
        Ok(MaybePendingBlockWithTxHashes::Block(block)) => {
            for tx_hash in block.transactions {
                println!("{tx_hash}");
            }
        }
        Ok(MaybePendingBlockWithTxHashes::PendingBlock(block)) => {
            println!("Pending block found - handling pending transactions may vary");
        }
        Err(e) => {
            println!("Failed to fetch the latest block: {e:?}");
        }
    }
}
