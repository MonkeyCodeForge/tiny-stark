pub mod types;
pub mod utils;

#[cfg(feature = "sqlxdb")]
pub mod sqlx;
#[cfg(feature = "sqlxdb")]
pub use sqlx::DefaultSqlxStorage;

use crate::storage::types::{
    BlockInfo, ContractInfo, ContractType, StorageError, TokenEvent, TokenInfo, TokenMintInfo, MemecoinCreatedEvent
};
use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait Storage {
    async fn register_mint(
        &self,
        contract_address: &str,
        token_id_hex: &str,
        info: &TokenMintInfo,
    ) -> Result<(), StorageError>;

    async fn register_token(
        &self,
        token: &TokenInfo,
        block_timestamp: u64,
    ) -> Result<(), StorageError>;

    async fn register_event(
        &self,
        event: &TokenEvent,
        block_timestamp: u64,
    ) -> Result<(), StorageError>;

    async fn get_contract_type(&self, contract_address: &str)
        -> Result<ContractType, StorageError>;

    async fn register_contract_info(
        &self,
        info: &ContractInfo,
        block_timestamp: u64,
    ) -> Result<(), StorageError>;

    async fn register_memecoin_created_event(
        &self,
        event: &MemecoinCreatedEvent,
        block_timestamp: u64) -> Result<(), StorageError>;


    /// A block info is only set if the block has a number and a timestamp.
    async fn set_block_info(
        &self,
        block_number: u64,
        block_timestamp: u64,
        info: BlockInfo,
    ) -> Result<(), StorageError>;

    async fn get_block_info(&self, block_number: u64) -> Result<BlockInfo, StorageError>;

    /// The block timestamps is always present. But the number can be missing
    /// for the pending block support.
    async fn clean_block(
        &self,
        block_timestamp: u64,
        block_number: Option<u64>,
    ) -> Result<(), StorageError>;
}
