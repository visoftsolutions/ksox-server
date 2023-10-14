use async_trait::async_trait;
use ethers::{
    providers::{Middleware, Provider},
    types::{Address, Transaction, TxHash, U256},
};
use serde::{Deserialize, Serialize};
use surrealdb::{Connection, Surreal};
use url::Url;

use crate::{
    errors::EvmNetworkError,
    traits::{EvmNetworkApi, EvmNetworkChecks, SurrealdbModel},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EvmNetwork {
    network_name: String,
    rpc_url: Url,
}
impl EvmNetwork {
    pub fn new(network_name: String, rpc_url: Url) -> Self {
        Self {
            network_name,
            rpc_url,
        }
    }
}

#[async_trait]
impl EvmNetworkApi for EvmNetwork {
    fn get_network_name(self) -> String {
        self.network_name
    }
    fn get_rpc_url(self) -> Url {
        self.rpc_url
    }
    async fn get_chain_id(self) -> Result<U256, EvmNetworkError> {
        let provider = Provider::try_from(self.rpc_url.as_str())?;
        Ok(provider.get_chainid().await?)
    }
    async fn get_transaction(
        self,
        transaction_hash: TxHash,
    ) -> Result<Transaction, EvmNetworkError> {
        let provider = Provider::try_from(self.rpc_url.as_str())?;
        Ok(provider
            .get_transaction(transaction_hash)
            .await?
            .ok_or_else(|| EvmNetworkError::Transaction)?)
    }
}

#[async_trait]
impl EvmNetworkChecks for EvmNetwork {
    async fn check_transaction(self, transaction_hash: TxHash, address: Address) -> bool {
        let transaction = match self.get_transaction(transaction_hash).await {
            Ok(transaction) => transaction,
            Err(err) => {
                tracing::error!("{:?}", err);
                return false;
            }
        };
        transaction.from == address
    }
}

#[async_trait]
impl SurrealdbModel for EvmNetwork {
    fn table_name() -> String {
        "evm-network".to_string()
    }
    async fn get_by_name<C: Connection>(
        db: Surreal<C>,
        name: &str,
    ) -> Result<Option<Self>, EvmNetworkError> {
        Ok(db
            .query("SELECT * FROM $table WHERE network_name = $name")
            .bind(("table", Self::table_name()))
            .bind(("name", name))
            .await?
            .take(0)?)
    }
}
