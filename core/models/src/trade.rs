use uuid::Uuid;

pub struct TradeRaw {
    pub id: Uuid,
    pub base_asset_id: Uuid,
    pub base_asset_volume: i64,
    pub quote_asset_id: Uuid,
    pub quote_asset_volume: i64,
}
