use ethers::providers::ProviderError;
use url::ParseError;

#[derive(Debug, thiserror::Error)]
pub enum EvmNetworkError {
    #[error("parse error")]
    Parse(#[from] ParseError),

    #[error("provider error")]
    Provider(#[from] ProviderError),

    #[error("transaction error")]
    Transaction,

    #[error("surrealdb error")]
    Surrealdb(#[from] surrealdb::Error),
}
