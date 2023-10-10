use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use chrono::Utc;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::errors::AuthError;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret =
        std::env::var("KSOX_SERVER_JWT_SECRET").expect("KSOX_SERVER_JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});
pub struct Keys {
    pub decoding: DecodingKey,
    pub encoding: EncodingKey,
}
impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            decoding: DecodingKey::from_secret(secret),
            encoding: EncodingKey::from_secret(secret),
        }
    }
}

pub trait JwtEncodeDecode<T> {
    fn decode(token: &str) -> jsonwebtoken::errors::Result<TokenData<T>>;
    fn encode(&self) -> jsonwebtoken::errors::Result<String>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
impl JwtEncodeDecode<Self> for Claims {
    fn decode(token: &str) -> jsonwebtoken::errors::Result<TokenData<Self>> {
        decode::<Claims>(token, &KEYS.decoding, &Validation::new(Algorithm::HS256))
    }
    fn encode(&self) -> jsonwebtoken::errors::Result<String> {
        encode(&Header::new(Algorithm::HS256), self, &KEYS.encoding)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data

        let token_data = Claims::decode(bearer.token()).map_err(|_| AuthError::InvalidToken)?;

        if token_data.claims.exp
            <= usize::try_from(Utc::now().timestamp()).map_err(|_| AuthError::WrongCredentials)?
        {
            return Err(AuthError::WrongCredentials);
        }

        Ok(token_data.claims)
    }
}
