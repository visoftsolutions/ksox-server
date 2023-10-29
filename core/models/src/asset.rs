use uuid::Uuid;

use crate::network::Network;

pub struct AssetRaw {
    pub id: Uuid,
    pub name: String,
    pub symbol: String,
    pub precision: i64,
    pub network: Network,
}
