use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{decode, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    sql::Uuid,
    Surreal,
};

use crate::{
    errors::AuthError,
    jwt::{Claims, KEYS},
};

static _SURREALDB_URL: Lazy<String> = Lazy::new(|| {
    std::env::var("KSOX_SERVER_SURREALDB_URL").expect("KSOX_SERVER_SURREALDB_URL must be set")
});

pub async fn _get_surrealdb() -> surrealdb::Result<Surreal<Client>> {
    Surreal::new::<Ws>(_SURREALDB_URL.to_string()).await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId(Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for UserId
where
    Surreal<Client>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data

        let _token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        let db = Surreal::<Client>::from_ref(state);
        let _groups = db.query("").bind(("table", "users"));

        Ok(UserId(Uuid::new()))
    }
}
