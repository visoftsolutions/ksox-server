#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("parse address error")]
    AddressParse(#[from] std::net::AddrParseError),

    #[error("axum server error")]
    Hyper(#[from] hyper::Error),

    #[error("tracing setup error")]
    Tracing(#[from] tracing::subscriber::SetGlobalDefaultError),
}
