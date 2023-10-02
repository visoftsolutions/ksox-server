use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("parse address error")]
    AddressParse(#[from] std::net::AddrParseError),

    #[error("axum server error")]
    Hyper(#[from] hyper::Error),

    #[error("tracing setup error")]
    Tracing(#[from] tracing::subscriber::SetGlobalDefaultError),
}

#[derive(Debug, Deserialize, Serialize)]
struct AuthErrorResponse {
    error: String,
}
#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(AuthErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}
