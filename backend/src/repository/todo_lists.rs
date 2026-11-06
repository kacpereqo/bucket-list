use sqlx::SqlitePool;

use crate::models::todo_lists::{TodoList, TodoListCreate, TodoListUpdate};

pub async fn get_by_bucket_list_id(
    pool: &SqlitePool,
    bucket_list_id: i64,
) -> Result<Vec<TodoList>, sqlx::Error> {
    sqlx::query_as!(
        TodoList,
        "SELECT * FROM todo_lists WHERE bucket_list_id = ?",
        bucket_list_id
    )
    .fetch_all(pool)
    .await
}

pub async fn create(pool: &SqlitePool, todo_list: TodoListCreate) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO todo_lists (bucket_list_id, name) VALUES (?, ?)",
        todo_list.bucket_list_id,
        todo_list.name
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn delete_by_id(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    let result = sqlx::query!("DELETE FROM todo_lists WHERE id = ?", id)
        .execute(pool)
        .await?;

    (result.rows_affected() > 0)
        .then_some(())
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn update_by_id(
    pool: &SqlitePool,
    id: i64,
    todo_list: TodoListUpdate,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE todo_lists SET name = ? WHERE id = ?",
        todo_list.name,
        id
    )
    .execute(pool)
    .await?;

    (result.rows_affected() > 0)
        .then_some(())
        .ok_or(sqlx::Error::RowNotFound)
}
