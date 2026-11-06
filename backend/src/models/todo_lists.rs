use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TodoList {
    pub id: i64,
    pub bucket_list_id: i64,
    pub name: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TodoListCreate {
    pub bucket_list_id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TodoListUpdate {
    pub name: String,
}
