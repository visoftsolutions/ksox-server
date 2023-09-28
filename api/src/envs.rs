use std::net::SocketAddr;

use url::Url;

use crate::errors;

pub struct Envs {
    pub surrealdb_url: Url,
    pub redis_url: Url,
    pub api_bind: SocketAddr,
}

pub fn get_envs() -> Result<Envs, errors::EnvError> {
    Ok(Envs {
        surrealdb_url: std::env::var("KSOX_SERVER_SURREALDB_URL")?.parse()?,
        redis_url: std::env::var("KSOX_SERVER_REDIS_URL")?.parse()?,
        api_bind: std::env::var("KSOX_SERVER_API_BIND")?.parse()?,
    })
}
