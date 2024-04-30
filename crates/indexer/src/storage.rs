use async_trait::async_trait;
use pontos::storage::{
    types::{
        BlockIndexingStatus, BlockInfo, ContractInfo, ContractType, MemecoinCreatedEvent,
        StorageError, TokenEvent, TokenInfo, TokenMintInfo,
    },
    Storage,
};

pub struct DefaultStorage;

impl DefaultStorage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for DefaultStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Storage for DefaultStorage {
    async fn register_mint(
        &self,
        contract_address: &str,
        token_id_hex: &str,
        info: &TokenMintInfo,
    ) -> Result<(), StorageError> {
        println!(
            "Registering mint {} {} {:?}",
            contract_address, token_id_hex, info
        );
        Ok(())
    }

    async fn register_token(
        &self,
        token: &TokenInfo,
        _block_timestamp: u64,
    ) -> Result<(), StorageError> {
        println!("Registering token {:?}", token);
        Ok(())
    }

    async fn register_event(
        &self,
        event: &TokenEvent,
        _block_timestamp: u64,
    ) -> Result<(), StorageError> {
        println!("Registering event {:?}", event);
        Ok(())
    }

    async fn get_contract_type(
        &self,
        contract_address: &str,
    ) -> Result<ContractType, StorageError> {
        println!("Getting contract info for contract {}", contract_address);
        Ok(ContractType::Other)
    }

    async fn register_contract_info(
        &self,
        info: &ContractInfo,
        _block_timestamp: u64,
    ) -> Result<(), StorageError> {
        println!(
            "Registering contract info {} for contract {}",
            info.contract_type, info.contract_address
        );
        Ok(())
    }

    async fn set_block_info(
        &self,
        block_number: u64,
        _block_timestamp: u64,
        info: BlockInfo,
    ) -> Result<(), StorageError> {
        println!("Setting block info {:?} for block #{}", info, block_number);
        Ok(())
    }

    async fn get_block_info(&self, block_number: u64) -> Result<BlockInfo, StorageError> {
        println!("Getting block info for block #{}", block_number);
        Ok(BlockInfo {
            indexer_version: String::from("0.0.1"),
            indexer_identifier: String::from("v0"),
            status: BlockIndexingStatus::None,
            block_number,
        })
    }

    async fn clean_block(
        &self,
        _block_timestamp: u64,
        block_number: Option<u64>,
    ) -> Result<(), StorageError> {
        println!("Cleaning block #{:?}", block_number);
        Ok(())
    }

    async fn register_memecoin_created_event(
        &self,
        event: &MemecoinCreatedEvent,
        block_timestamp: u64,
    ) -> Result<(), StorageError> {
        println!(
            "Registering memecoin created event {:?} at {}",
            event, block_timestamp
        );
        Ok(())
    }
}
