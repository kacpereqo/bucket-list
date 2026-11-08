use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TodoItem {
    pub id: i64,
    pub todo_list_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub is_done: bool,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TodoItemCreate {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct TodoItemUpdate {
    pub title: String,
    pub description: Option<String>,
}
