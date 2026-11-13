use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::{todo_items::TodoItem, todo_lists::TodoList};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BucketList {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct BucketListCreate {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct BucketListUpdate {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct TodoListWithItems {
    #[serde(flatten)]
    pub list: TodoList,
    pub items: Vec<TodoItem>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BucketListWithLists {
    #[serde(flatten)]
    pub bucket: BucketList,
    pub todo_lists: Vec<TodoListWithItems>,
}
