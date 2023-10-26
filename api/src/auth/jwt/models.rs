use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use super::traits::JwtEncodeDecode;

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
