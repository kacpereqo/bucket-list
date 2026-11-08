use sqlx::SqlitePool;

use crate::models::todo_items::{TodoItem, TodoItemCreate, TodoItemUpdate};

pub async fn get_by_todo_list_id(
    pool: &SqlitePool,
    todo_list_id: i64,
) -> Result<Vec<TodoItem>, sqlx::Error> {
    sqlx::query_as!(
        TodoItem,
        "SELECT * FROM todo_items WHERE todo_list_id = ?",
        todo_list_id
    )
    .fetch_all(pool)
    .await
}

pub async fn create(
    pool: &SqlitePool,
    todo_list_id: i64,
    todo_item: TodoItemCreate,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO todo_items (todo_list_id, title, description) VALUES (?,?,?)",
        todo_list_id,
        todo_item.title,
        todo_item.description,
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn update_toggle_done_by_id(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE todo_items SET is_done = NOT is_done WHERE id = ?",
        id
    )
    .execute(pool)
    .await?;

    (result.rows_affected() > 0)
        .then_some(())
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn delete_by_id(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    let result = sqlx::query!("DELETE FROM todo_items WHERE id = ?", id)
        .execute(pool)
        .await?;

    (result.rows_affected() > 0)
        .then_some(())
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn update_by_id(
    pool: &SqlitePool,
    id: i64,
    todo_item: TodoItemUpdate,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE todo_items SET title = ?, description = ? WHERE id = ?",
        todo_item.title,
        todo_item.description,
        id
    )
    .execute(pool)
    .await?;

    (result.rows_affected() > 0)
        .then_some(())
        .ok_or(sqlx::Error::RowNotFound)
}
