use async_trait::async_trait;
use ethers::types::{Address, Transaction, TxHash, U256};
use serde::de::DeserializeOwned;
use surrealdb::{Connection, Surreal};
use url::Url;

use crate::errors::EvmNetworkError;

#[async_trait]
pub trait SurrealdbNamedModel: Sized + DeserializeOwned {
    fn table_name() -> String;
    async fn get_all<C: Connection>(db: Surreal<C>) -> Result<Vec<Self>, EvmNetworkError> {
        Ok(db.select(Self::table_name()).await?)
    }
    async fn get_by_name<C: Connection>(
        db: Surreal<C>,
        name: &str,
    ) -> Result<Option<Self>, EvmNetworkError> {
        Ok(db
            .query("SELECT * FROM $table WHERE name = $name")
            .bind(("table", Self::table_name()))
            .bind(("name", name))
            .await?
            .take(0)?)
    }
}

#[async_trait]
pub trait EvmNetworkApi {
    fn get_name(self) -> String;
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
