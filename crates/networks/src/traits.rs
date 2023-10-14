use async_trait::async_trait;
use ethers::types::{Address, Transaction, TxHash, U256};
use surrealdb::{Connection, Surreal};
use url::Url;

use crate::errors::EvmNetworkError;

#[async_trait]
pub trait SurrealdbModel: Sized {
    fn table_name() -> String;
    async fn get_by_name<C: Connection>(
        db: Surreal<C>,
        name: &str,
    ) -> Result<Option<Self>, EvmNetworkError>;
}

#[async_trait]
pub trait EvmNetworkApi {
    fn get_network_name(self) -> String;
    fn get_rpc_url(self) -> Url;
    async fn get_chain_id(self) -> Result<U256, EvmNetworkError>;
    async fn get_transaction(
        self,
        transaction_hash: TxHash,
    ) -> Result<Transaction, EvmNetworkError>;
}

#[async_trait]
pub trait EvmNetworkChecks {
    async fn check_transaction(self, transaction_hash: TxHash, address: Address) -> bool;
}
