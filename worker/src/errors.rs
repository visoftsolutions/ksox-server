#[derive(Debug, thiserror::Error)]
pub enum WorkerError {
    #[error("axum server error")]
    HyperError(#[from] hyper::Error),

    #[error("environment variable error")]
    EnvVarError(#[from] std::env::VarError),

    #[error("parse address error")]
    AddressParseError(#[from] std::net::AddrParseError),

    #[error("tracing setup error")]
    TracingError(#[from] tracing::subscriber::SetGlobalDefaultError),
}
