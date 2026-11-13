mod common;

use actix_web::{
    http::{header::ContentType, StatusCode},
    test,
};
use sqlx::SqlitePool;

#[sqlx::test]
async fn update_changes_title_and_description(pool: SqlitePool) {
    let id = common::seed_todo_item(&pool).await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::put()
        .uri(&format!("/api/items/{id}"))
        .insert_header(ContentType::json())
        .set_payload(r#"{"title": "New title", "description": "New description"}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    let (title, description): (String, Option<String>) =
        sqlx::query_as("SELECT title, description FROM todo_items WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(title, "New title");
    assert_eq!(description.as_deref(), Some("New description"));
}

#[sqlx::test]
async fn update_missing_item_returns_404(pool: SqlitePool) {
    let app = common::app(&pool).await;

    let req = test::TestRequest::put()
        .uri("/api/items/999")
        .insert_header(ContentType::json())
        .set_payload(r#"{"title": "x", "description": null}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn update_without_title_returns_400(pool: SqlitePool) {
    let id = common::seed_todo_item(&pool).await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::put()
        .uri(&format!("/api/items/{id}"))
        .insert_header(ContentType::json())
        .set_payload(r#"{"description": "no title"}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test]
async fn toggle_flips_is_done_both_ways(pool: SqlitePool) {
    let id = common::seed_todo_item(&pool).await;
    let app = common::app(&pool).await;

    for expected in [true, false] {
        let req = test::TestRequest::patch()
            .uri(&format!("/api/items/{id}/toggle"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        let is_done: bool = sqlx::query_scalar("SELECT is_done FROM todo_items WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(is_done, expected);
    }
}

#[sqlx::test]
async fn toggle_missing_item_returns_404(pool: SqlitePool) {
    let app = common::app(&pool).await;

    let req = test::TestRequest::patch()
        .uri("/api/items/999/toggle")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn delete_removes_item(pool: SqlitePool) {
    let id = common::seed_todo_item(&pool).await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::delete()
        .uri(&format!("/api/items/{id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM todo_items WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}

#[sqlx::test]
async fn delete_missing_item_returns_404(pool: SqlitePool) {
    let app = common::app(&pool).await;

    let req = test::TestRequest::delete()
        .uri("/api/items/999")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
