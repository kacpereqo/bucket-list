use sqlx::SqlitePool;

use crate::models::users::{User, UserCreate, UserUpdate};

pub async fn get_by_id(pool: &SqlitePool, id: i64) -> Result<User, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
        .fetch_one(pool)
        .await
}

pub async fn create(pool: &SqlitePool, user: UserCreate) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO users (name, email, profile_img_url) VALUES (?, ?, ?)",
        user.name,
        user.email,
        user.profile_img_url
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn delete_by_id(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(pool)
        .await?;

    (result.rows_affected() > 0)
        .then_some(())
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn update_by_id(pool: &SqlitePool, id: i64, user: UserUpdate) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE users SET name = ?, profile_img_url = ? WHERE id = ?",
        user.name,
        user.profile_img_url,
        id
    )
    .execute(pool)
    .await?;

    (result.rows_affected() > 0)
        .then_some(())
        .ok_or(sqlx::Error::RowNotFound)
}
