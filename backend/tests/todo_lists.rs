mod common;

use actix_web::{
    http::{header::ContentType, StatusCode},
    test,
};
use serde_json::Value;
use sqlx::SqlitePool;

#[sqlx::test]
async fn get_items_returns_items_of_the_list(pool: SqlitePool) {
    let list_id = common::seed_todo_list(&pool).await;
    common::insert_todo_item(&pool, list_id).await;
    common::insert_todo_item(&pool, list_id).await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/lists/{list_id}/items"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    let items = body.as_array().expect("response should be a JSON array");
    assert_eq!(items.len(), 2);
    assert!(items.iter().all(|item| item["todo_list_id"] == list_id));
}

#[sqlx::test]
async fn get_items_of_empty_list_returns_empty_array(pool: SqlitePool) {
    let list_id = common::seed_todo_list(&pool).await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/lists/{list_id}/items"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body, serde_json::json!([]));
}

#[sqlx::test]
async fn create_item_adds_item_to_the_list(pool: SqlitePool) {
    let list_id = common::seed_todo_list(&pool).await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::post()
        .uri(&format!("/api/lists/{list_id}/items"))
        .insert_header(ContentType::json())
        .set_payload(r#"{"title": "Buy tickets", "description": null}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);
    let new_id: i64 = test::read_body_json(resp).await;

    let (todo_list_id, title, is_done): (i64, String, bool) =
        sqlx::query_as("SELECT todo_list_id, title, is_done FROM todo_items WHERE id = ?")
            .bind(new_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(todo_list_id, list_id);
    assert_eq!(title, "Buy tickets");
    assert!(!is_done);
}
