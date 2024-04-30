use starknet::{
    core::types::{BlockId, BlockTag, FieldElement, EventFilter},
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider
    },
};
use starknet::macros::selector;
use dotenv::dotenv;
use url::Url;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-mainnet.infura.io/v3/bd221cae8220402f88b090b4132679f3").unwrap(),
    ));
    

    let event_filter = EventFilter {
        from_block: Some(BlockId::Number(601824)),
        to_block: Some(BlockId::Tag(BlockTag::Latest)),
        address: FieldElement::from_hex_be("01a46467a9246f45c8c340f1f155266a26a71c07bd55d36e8d1c7d0d438a2dbc").ok(),
        keys:  Some(vec![vec![
            selector!("MemecoinCreated")
        ]])
    };

    let events_result = provider.get_events(event_filter, None, 100).await;

    match events_result {
        Ok(events_page) => {
            for event in events_page.events {
                println!("Event found: {:?}", event);
            }
        },
        Err(e) => {
            println!("Failed to fetch events: {:?}", e);
        }
    }
}
