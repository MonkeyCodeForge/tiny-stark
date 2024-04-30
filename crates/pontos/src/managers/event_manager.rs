use crate::storage::types::{ContractInfo, ContractType, EventType, MemecoinCreatedEvent};
use crate::storage::Storage;
use anyhow::{anyhow, Result};
use log::info;
use starknet::core::types::{EmittedEvent, FieldElement};
use starknet::core::utils::starknet_keccak;
use starknet::macros::selector;
use std::sync::Arc;
use tiny_starknet::{format::to_hex_str, CairoU256};
use tracing::debug;

const TRANSFER_SELECTOR: FieldElement = selector!("Transfer");
const MEMECOINCREATED_SELECTOR: FieldElement = selector!("MemecoinCreated");

#[derive(Debug)]
pub struct EventManager<S: Storage> {
    storage: Arc<S>,
}

impl<S: Storage> EventManager<S> {
    /// Initializes a new instance.
    pub fn new(storage: Arc<S>) -> Self {
        EventManager {
            storage: Arc::clone(&storage),
        }
    }

    /// Returns the selectors used to filter events.
    pub fn keys_selector(&self) -> Option<Vec<Vec<FieldElement>>> {
        Some(vec![vec![TRANSFER_SELECTOR, MEMECOINCREATED_SELECTOR]])
    }

    pub async fn format_and_register_event(
        &self,
        event: &EmittedEvent,
        block_timestamp: u64,
    ) -> Result<()> {
        debug!(
            "Processing event: event={:?}, timestamp={}",
            event, block_timestamp
        );

        match event.keys.first() {
            Some(event_selector) => {
                if event_selector == &MEMECOINCREATED_SELECTOR {
                    info!("MemecoinCreated event detected");

                    let event_info = Self::get_memecoin_created_info_from_felts(&event.data)
                        .ok_or_else(|| anyhow!("Invalid data for MemecoinCreated event"))?;
                    let (owner, name, symbol, initial_supply, memecoin_address) = event_info;

                    self.storage
                        .register_contract_info(
                            &ContractInfo {
                                contract_address: to_hex_str(&memecoin_address),
                                contract_type: ContractType::UNRUGGABLE,
                                name: Some(
                                    String::from_utf8_lossy(&name.to_bytes_be()).into_owned(),
                                ),
                                symbol: Some(
                                    String::from_utf8_lossy(&symbol.to_bytes_be()).into_owned(),
                                ),
                                image: None,
                            },
                            block_timestamp,
                        )
                        .await?;

                    // self.storage
                    //     .register_memecoin_created_event(
                    //         &MemecoinCreatedEvent {
                    //             owner: to_hex_str(&owner),
                    //             name: String::from_utf8_lossy(&name.to_bytes_be()).into_owned(),
                    //             symbol: String::from_utf8_lossy(&symbol.to_bytes_be()).into_owned(),
                    //             initial_supply,
                    //             memecoin_address: to_hex_str(&memecoin_address),
                    //         },
                    //         block_timestamp,
                    //     )
                    //     .await?;
                }
            }
            _ => {}
        };

        Ok(())
    }

    pub fn get_event_type(from: FieldElement, to: FieldElement) -> EventType {
        if from == FieldElement::ZERO {
            EventType::Mint
        } else if to == FieldElement::ZERO {
            EventType::Burn
        } else {
            EventType::Transfer
        }
    }

    /// Returns the event id as a field element.
    /// We enforce everything to be a field element to have fix
    /// bytes lengths, and ease the re-computation of this value
    /// from else where.
    pub fn get_event_id(
        token_id: &CairoU256,
        from: &FieldElement,
        to: &FieldElement,
        timestamp: u64,
        event: &EmittedEvent,
    ) -> FieldElement {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&FieldElement::from(token_id.low).to_bytes_be());
        bytes.extend_from_slice(&FieldElement::from(token_id.high).to_bytes_be());
        bytes.extend_from_slice(&from.to_bytes_be());
        bytes.extend_from_slice(&to.to_bytes_be());
        bytes.extend_from_slice(&event.from_address.to_bytes_be());
        bytes.extend_from_slice(&event.transaction_hash.to_bytes_be());
        bytes.extend_from_slice(&FieldElement::from(timestamp).to_bytes_be());
        starknet_keccak(&bytes)
    }

    /// Returns the event info from vector of felts.
    /// Event info are (from, to, token_id).
    ///
    /// This methods considers that the info of the
    /// event is starting at index 0 of the input vector.
    fn get_event_info_from_felts(
        felts: &[FieldElement],
    ) -> Option<(FieldElement, FieldElement, CairoU256)> {
        if felts.len() < 4 {
            return None;
        }
        let from = felts[0];
        let to = felts[1];

        // Safe to unwrap, as emitted events follow cairo sequencer specification.
        let token_id = CairoU256 {
            low: felts[2].try_into().unwrap(),
            high: felts[3].try_into().unwrap(),
        };

        Some((from, to, token_id))
    }

    /// helper method to parse the event details
    fn get_memecoin_created_info_from_felts(
        felts: &[FieldElement],
    ) -> Option<(
        FieldElement,
        FieldElement,
        FieldElement,
        CairoU256,
        FieldElement,
    )> {
        if felts.len() < 5 {
            return None;
        }
        let owner = felts[0];
        let name = felts[1];
        let symbol = felts[2];
        let initial_supply = CairoU256 {
            low: felts[3].try_into().unwrap(),
            high: felts[4].try_into().unwrap(),
        };
        let memecoin_address = felts[5];
        Some((owner, name, symbol, initial_supply, memecoin_address))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::MockStorage;

    /// Sets up sample data and event for testing purposes.
    fn setup_sample_event() -> EmittedEvent {
        EmittedEvent {
            from_address: FieldElement::from_hex_be("0x0").unwrap(),
            block_hash: Some(FieldElement::from_dec_str("786").unwrap()),
            transaction_hash: FieldElement::from_dec_str("5432").unwrap(),
            block_number: Some(111),
            keys: vec![
                TRANSFER_SELECTOR,
                FieldElement::from_hex_be("0x1234").unwrap(),
                FieldElement::from_hex_be("0x5678").unwrap(),
            ],
            data: vec![
                FieldElement::from_hex_be("0x1234").unwrap(),
                FieldElement::from_hex_be("0x5678").unwrap(),
                FieldElement::from_dec_str("91011").unwrap(),
                FieldElement::from_dec_str("121314").unwrap(),
            ],
        }
    }

    #[test]
    fn test_keys_selector() {
        let storage = Arc::new(MockStorage::default());
        let manager = EventManager::new(storage);

        // Call the method
        let result = manager.keys_selector().unwrap();

        // Define expected result
        let expected = vec![vec![selector!("Transfer"), selector!("MemecoinCreated")]];

        // Assert the output
        assert_eq!(result, expected);
    }

    /// Tests the `get_event_info_from_felts` method with correct input format and length.
    /// Ensures that the method correctly extracts and returns the event info.
    #[test]
    fn test_get_event_info_from_felts() {
        // Create sample data for the test
        let from_value = FieldElement::from_dec_str("1234").unwrap();
        let to_value = FieldElement::from_dec_str("5678").unwrap();
        let token_id_low = 91011_u128;
        let token_id_high = 121314_u128;

        let sample_data = vec![
            from_value,
            to_value,
            token_id_low.into(),
            token_id_high.into(),
        ];

        // Call the method
        let result = EventManager::<MockStorage>::get_event_info_from_felts(&sample_data);

        // Assert the output
        assert_eq!(result.is_some(), true);
        let (from, to, token_id) = result.unwrap();
        assert_eq!(from, from_value);
        assert_eq!(to, to_value);
        assert_eq!(token_id.low, token_id_low);
        assert_eq!(token_id.high, token_id_high);
    }

    /// Tests the `get_event_info_from_felts` method with insufficient FieldElements.
    /// Ensures that the method returns None when not provided enough data.
    #[test]
    fn test_get_event_info_from_felts_insufficient_data() {
        // Create sample data for the test with insufficient FieldElements
        let sample_data = vec![
            FieldElement::from_dec_str("1234").unwrap(),
            FieldElement::from_dec_str("5678").unwrap(),
        ];

        // Call the method
        let result = EventManager::<MockStorage>::get_event_info_from_felts(&sample_data);

        // Assert the output
        assert_eq!(result.is_none(), true);
    }
}
