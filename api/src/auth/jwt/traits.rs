use jsonwebtoken::TokenData;

pub trait JwtEncodeDecode<T> {
    fn decode(token: &str) -> jsonwebtoken::errors::Result<TokenData<T>>;
    fn encode(&self) -> jsonwebtoken::errors::Result<String>;
}
