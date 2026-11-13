#![allow(dead_code)]

use actix_web::{
    body::MessageBody,
    dev::{Service, ServiceResponse},
    test, Error,
};
use sqlx::SqlitePool;

/// Builds the real app (`backend::build_app`, same as `main.rs`) on top of the given pool.
pub async fn app(
    pool: &SqlitePool,
) -> impl Service<actix_http::Request, Response = ServiceResponse<impl MessageBody>, Error = Error> {
    test::init_service(backend::build_app(pool.clone())).await
}

pub async fn insert_user(pool: &SqlitePool, email: &str) -> i64 {
    sqlx::query("INSERT INTO users (name, email) VALUES ('Test User', ?)")
        .bind(email)
        .execute(pool)
        .await
        .unwrap()
        .last_insert_rowid()
}

pub async fn insert_bucket_list(pool: &SqlitePool, owner_id: i64) -> i64 {
    sqlx::query("INSERT INTO bucket_lists (owner_id, name) VALUES (?, 'Test bucket list')")
        .bind(owner_id)
        .execute(pool)
        .await
        .unwrap()
        .last_insert_rowid()
}

pub async fn insert_todo_list(pool: &SqlitePool, bucket_list_id: i64) -> i64 {
    sqlx::query("INSERT INTO todo_lists (bucket_list_id, name) VALUES (?, 'Test todo list')")
        .bind(bucket_list_id)
        .execute(pool)
        .await
        .unwrap()
        .last_insert_rowid()
}

pub async fn insert_todo_item(pool: &SqlitePool, todo_list_id: i64) -> i64 {
    sqlx::query("INSERT INTO todo_items (todo_list_id, title) VALUES (?, 'Test item')")
        .bind(todo_list_id)
        .execute(pool)
        .await
        .unwrap()
        .last_insert_rowid()
}

/// Inserts user → bucket list → todo list and returns the todo list id.
pub async fn seed_todo_list(pool: &SqlitePool) -> i64 {
    let user_id = insert_user(pool, "test@example.com").await;
    let bucket_list_id = insert_bucket_list(pool, user_id).await;
    insert_todo_list(pool, bucket_list_id).await
}

/// Inserts user → bucket list → todo list → todo item and returns the item id.
pub async fn seed_todo_item(pool: &SqlitePool) -> i64 {
    let todo_list_id = seed_todo_list(pool).await;
    insert_todo_item(pool, todo_list_id).await
}
