//! Tests for app-wide setup in `backend::build_app`: compression, OpenAPI, health.

mod common;

use actix_web::{
    http::{header, StatusCode},
    test,
};
use backend::auth::USER_ID_HEADER;
use serde_json::Value;
use sqlx::SqlitePool;

async fn seed_overview(pool: &SqlitePool) -> i64 {
    let me = common::insert_user(pool, "me@example.com").await;
    for _ in 0..5 {
        let bucket = common::insert_bucket_list(pool, me).await;
        let list = common::insert_todo_list(pool, bucket).await;
        for _ in 0..10 {
            common::insert_todo_item(pool, list).await;
        }
    }
    me
}

#[sqlx::test]
async fn responses_are_gzipped_when_client_accepts_gzip(pool: SqlitePool) {
    let me = seed_overview(&pool).await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::get()
        .uri("/api/me/overview")
        .insert_header((USER_ID_HEADER, me.to_string()))
        .insert_header((header::ACCEPT_ENCODING, "gzip"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        resp.headers().get(header::CONTENT_ENCODING).unwrap(),
        "gzip"
    );
}

#[sqlx::test]
async fn responses_are_not_compressed_without_accept_encoding(pool: SqlitePool) {
    let me = seed_overview(&pool).await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::get()
        .uri("/api/me/overview")
        .insert_header((USER_ID_HEADER, me.to_string()))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    assert!(resp.headers().get(header::CONTENT_ENCODING).is_none());
}

#[sqlx::test]
async fn openapi_spec_lists_all_endpoints(pool: SqlitePool) {
    let app = common::app(&pool).await;

    let req = test::TestRequest::get()
        .uri("/api-docs/openapi.json")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let spec: Value = test::read_body_json(resp).await;
    let expected = [
        ("/api/health/db", "get"),
        ("/api/me/buckets", "get"),
        ("/api/me/overview", "get"),
        ("/api/buckets", "post"),
        ("/api/buckets/{id}", "put"),
        ("/api/lists/{id}/items", "get"),
        ("/api/lists/{id}/items", "post"),
        ("/api/items/{id}", "put"),
        ("/api/items/{id}", "delete"),
        ("/api/items/{id}/toggle", "patch"),
    ];
    for (path, method) in expected {
        assert!(
            spec["paths"][path][method].is_object(),
            "{} {} missing from OpenAPI spec",
            method.to_uppercase(),
            path
        );
    }
}

#[sqlx::test]
async fn swagger_ui_is_served(pool: SqlitePool) {
    let app = common::app(&pool).await;

    let req = test::TestRequest::get().uri("/docs/").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[sqlx::test]
async fn health_db_returns_epoch(pool: SqlitePool) {
    let app = common::app(&pool).await;

    let req = test::TestRequest::get().uri("/api/health/db").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert!(body["epoch"].as_i64().unwrap() > 0);
}
