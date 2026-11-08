use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BucketList {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct BucketListCreate {
    pub owner_id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct BucketListUpdate {
    pub name: String,
}
