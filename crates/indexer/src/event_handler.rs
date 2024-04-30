use async_trait::async_trait;
use pontos::{
    event_handler::EventHandler,
    storage::types::{TokenEvent, TokenInfo},
};

// Default event hanlder.
pub struct DefaultEventHandler;

impl DefaultEventHandler {
    pub fn new() -> Self {
        DefaultEventHandler {}
    }
}

#[async_trait]
impl EventHandler for DefaultEventHandler {
    async fn on_block_processed(&self, block_number: u64, indexation_progress: f64) {
        println!(
            "pontos: block processed: block_number={}, indexation_progress={}",
            block_number, indexation_progress
        );
    }

    async fn on_indexation_range_completed(&self) {
        println!("pontos: indexation range completed");
    }

    async fn on_new_latest_block(&self, block_number: u64) {
        println!("pontos: new latest block {:?}", block_number);
    }

    async fn on_block_processing(&self, block_timestamp: u64, block_number: Option<u64>) {
        // TODO: here we want to call some storage if needed from an other object.
        // But it's totally unrelated to the core process, so we can do whatever we want here.
        println!(
            "pontos: processing block: block_timestamp={}, block_number={:?}",
            block_timestamp, block_number
        );
    }

    async fn on_token_registered(&self, token: TokenInfo) {
        println!("pontos: token registered {:?}", token);
    }

    async fn on_event_registered(&self, event: TokenEvent) {
        println!("pontos: event registered {:?}", event);
    }
}
