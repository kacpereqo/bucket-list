use sqlx::SqlitePool;

use crate::models::bucket_lists::{BucketList, BucketListCreate, BucketListUpdate};

pub async fn get_shared_with_user_id(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<BucketList>, sqlx::Error> {
    sqlx::query_as!(
        BucketList,
        "SELECT bucket_lists.* FROM bucket_lists
         JOIN shared_bucket_lists ON shared_bucket_lists.bucket_list_id = bucket_lists.id
         WHERE shared_bucket_lists.user_id = ?",
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn create(pool: &SqlitePool, bucket_list: BucketListCreate) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO bucket_lists (owner_id, name) VALUES (?, ?)",
        bucket_list.owner_id,
        bucket_list.name
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn delete_by_id(pool: &SqlitePool, id: i64, user_id: i64) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM bucket_lists WHERE id = ? AND owner_id = ?",
        id,
        user_id
    )
    .execute(pool)
    .await?;

    (result.rows_affected() > 0)
        .then_some(())
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn update_by_id(
    pool: &SqlitePool,
    id: i64,
    user_id: i64,
    bucket_list: BucketListUpdate,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE bucket_lists SET name = ? WHERE id = ? AND owner_id = ?",
        bucket_list.name,
        id,
        user_id
    )
    .execute(pool)
    .await?;

    (result.rows_affected() > 0)
        .then_some(())
        .ok_or(sqlx::Error::RowNotFound)
}
