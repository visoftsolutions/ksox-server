use uuid::Uuid;

pub struct BalanceRaw {
    pub id: Uuid,
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub value: i64,
}
