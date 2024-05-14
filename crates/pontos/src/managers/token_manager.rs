use crate::storage::types::{EventType, TokenEvent, TokenInfo, TokenMintInfo};
use crate::storage::Storage;
use anyhow::{anyhow, Result};
use starknet::core::types::*;
use starknet::macros::selector;
use std::sync::Arc;
use tiny_starknet::client::StarknetClient;
use tiny_starknet::format::to_hex_str;
use tiny_starknet::CairoU256;

#[derive(Debug)]
pub struct TokenManager<S: Storage, C: StarknetClient> {
    storage: Arc<S>,
    client: Arc<C>,
}

impl<S: Storage, C: StarknetClient> TokenManager<S, C> {
    /// Initializes a new instance.
    pub fn new(storage: Arc<S>, client: Arc<C>) -> Self {
        Self {
            storage: Arc::clone(&storage),
            client: Arc::clone(&client),
        }
    }

    /// Formats a token registry from the token event data.
    pub async fn format_and_register_token(
        &self,
        token_id: &CairoU256,
        event: &TokenEvent,
        block_timestamp: u64,
        block_number: Option<u64>,
    ) -> Result<()> {
        let mut token = TokenInfo {
            contract_address: event.contract_address.clone(),
            token_id: event.token_id.clone(),
            token_id_hex: event.token_id_hex.clone(),
            ..Default::default()
        };

        let token_owner_raw_result = self
            .get_token_owner(
                FieldElement::from_hex_be(&event.contract_address)
                    .expect("Contract address bad format"),
                token_id.low.into(),
                token_id.high.into(),
            )
            .await;

        token.owner = token_owner_raw_result
            .ok()
            .and_then(|owner| owner.first().map(to_hex_str))
            .unwrap_or_default();

        self.storage.register_token(&token, block_timestamp).await?;

        if event.event_type == EventType::Mint {
            let info = TokenMintInfo {
                address: event.to_address.clone(),
                timestamp: event.timestamp,
                transaction_hash: event.transaction_hash.clone(),
                block_number,
            };

            self.storage
                .register_mint(&token.contract_address, &token.token_id_hex, &info)
                .await?;
        }

        Ok(())
    }

    /// Retrieves the token owner for the last block.
    pub async fn get_token_owner(
        &self,
        contract_address: FieldElement,
        token_id_low: FieldElement,
        token_id_high: FieldElement,
    ) -> Result<Vec<FieldElement>> {
        let block = BlockId::Tag(BlockTag::Pending);
        let selectors = vec![selector!("owner_of"), selector!("ownerOf")];

        for selector in selectors {
            if let Ok(res) = self
                .client
                .call_contract(
                    contract_address,
                    selector,
                    vec![token_id_low, token_id_high],
                    block,
                )
                .await
            {
                return Ok(res);
            }
        }

        Err(anyhow!("Failed to get token owner from chain"))
    }
}
