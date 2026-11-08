use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub profile_img_url: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UserCreate {
    pub name: String,
    pub email: String,
    pub profile_img_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UserUpdate {
    pub name: String,
    pub profile_img_url: Option<String>,
}
