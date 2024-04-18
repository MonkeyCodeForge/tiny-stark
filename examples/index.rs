use starknet::{
    core::types::{BlockId, BlockTag},
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider
    },
};
use url::Url;

#[tokio::main]
async fn main() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-mainnet.infura.io/v3/bd221cae8220402f88b090b4132679f3").unwrap(),
    ));

    let latest_block = provider
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
        .await;
    println!("{latest_block:#?}");
}