#[derive(Debug, thiserror::Error)]
pub enum EnvError {
    #[error("environment variable error")]
    EnvVar(#[from] std::env::VarError),

    #[error("parse address error")]
    AddressParse(#[from] std::net::AddrParseError),

    #[error("parse url error")]
    UrlParse(#[from] url::ParseError),
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("axum server error")]
    Hyper(#[from] hyper::Error),

    #[error("environment variable error")]
    Envs(#[from] EnvError),

    #[error("tracing setup error")]
    Tracing(#[from] tracing::subscriber::SetGlobalDefaultError),
}
