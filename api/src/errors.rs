#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("axum server error")]
    Hyper(#[from] hyper::Error),

    #[error("environment variable error")]
    EnvVar(#[from] std::env::VarError),

    #[error("parse address error")]
    AddressParse(#[from] std::net::AddrParseError),

    #[error("tracing setup error")]
    Tracing(#[from] tracing::subscriber::SetGlobalDefaultError),
}
