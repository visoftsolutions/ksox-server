pub mod models;
pub mod traits;

#[cfg(test)]
mod tests;

use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use chrono::Utc;

use self::{models::Claims, traits::JwtEncodeDecode};

use super::errors::AuthError;

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
