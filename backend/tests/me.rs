mod common;

use actix_web::{http::StatusCode, test};
use backend::auth::USER_ID_HEADER;
use serde_json::Value;
use sqlx::SqlitePool;

async fn bucket_ids(body: Value) -> Vec<i64> {
    let mut ids: Vec<i64> = body
        .as_array()
        .expect("response should be a JSON array")
        .iter()
        .map(|bucket| bucket["id"].as_i64().unwrap())
        .collect();
    ids.sort();
    ids
}

#[sqlx::test]
async fn get_buckets_returns_owned_and_shared_lists_only(pool: SqlitePool) {
    let me = common::insert_user(&pool, "me@example.com").await;
    let other = common::insert_user(&pool, "other@example.com").await;

    let mine = common::insert_bucket_list(&pool, me).await;
    let shared_with_me = common::insert_bucket_list(&pool, other).await;
    let _not_mine = common::insert_bucket_list(&pool, other).await;

    sqlx::query("INSERT INTO shared_bucket_lists (user_id, bucket_list_id) VALUES (?, ?)")
        .bind(me)
        .bind(shared_with_me)
        .execute(&pool)
        .await
        .unwrap();

    let app = common::app(&pool).await;
    let req = test::TestRequest::get()
        .uri("/api/me/buckets")
        .insert_header((USER_ID_HEADER, me.to_string()))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(bucket_ids(body).await, vec![mine, shared_with_me]);
}

#[sqlx::test]
async fn get_buckets_without_header_returns_401(pool: SqlitePool) {
    let app = common::app(&pool).await;

    let req = test::TestRequest::get().uri("/api/me/buckets").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn get_buckets_with_invalid_header_returns_401(pool: SqlitePool) {
    let app = common::app(&pool).await;

    let req = test::TestRequest::get()
        .uri("/api/me/buckets")
        .insert_header((USER_ID_HEADER, "not-a-number"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn old_users_id_buckets_route_is_gone(pool: SqlitePool) {
    let user = common::insert_user(&pool, "me@example.com").await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/users/{user}/buckets"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn get_overview_returns_nested_buckets_lists_and_items(pool: SqlitePool) {
    let me = common::insert_user(&pool, "me@example.com").await;
    let other = common::insert_user(&pool, "other@example.com").await;

    let mine = common::insert_bucket_list(&pool, me).await;
    let list = common::insert_todo_list(&pool, mine).await;
    let item = common::insert_todo_item(&pool, list).await;
    let empty_bucket = common::insert_bucket_list(&pool, me).await;
    let hidden = common::insert_bucket_list(&pool, other).await;
    common::insert_todo_list(&pool, hidden).await;

    let app = common::app(&pool).await;
    let req = test::TestRequest::get()
        .uri("/api/me/overview")
        .insert_header((USER_ID_HEADER, me.to_string()))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    let buckets = body.as_array().expect("response should be a JSON array");
    assert_eq!(buckets.len(), 2);

    let first = &buckets[0];
    assert_eq!(first["id"], mine);
    assert_eq!(first["owner_id"], me);
    assert_eq!(first["todo_lists"][0]["id"], list);
    assert_eq!(first["todo_lists"][0]["items"][0]["id"], item);
    assert_eq!(first["todo_lists"][0]["items"][0]["is_done"], false);

    assert_eq!(buckets[1]["id"], empty_bucket);
    assert_eq!(buckets[1]["todo_lists"], serde_json::json!([]));
}

#[sqlx::test]
async fn get_overview_matches_repository_json_string(pool: SqlitePool) {
    let me = common::insert_user(&pool, "me@example.com").await;
    let bucket = common::insert_bucket_list(&pool, me).await;
    let list = common::insert_todo_list(&pool, bucket).await;
    common::insert_todo_item(&pool, list).await;
    common::insert_todo_list(&pool, bucket).await;

    let app = common::app(&pool).await;
    let req = test::TestRequest::get()
        .uri("/api/me/overview")
        .insert_header((USER_ID_HEADER, me.to_string()))
        .to_request();
    let body: Value = test::read_body_json(test::call_service(&app, req).await).await;

    let from_sql: Value = serde_json::from_str(
        &backend::repository::bucket_lists::get_accessible_with_lists_and_items_by_user_id_as_json_string(
            &pool, me,
        )
        .await
        .unwrap(),
    )
    .unwrap();
    assert_eq!(body, from_sql);
}

#[sqlx::test]
async fn get_overview_for_user_without_buckets_returns_empty_array(pool: SqlitePool) {
    let me = common::insert_user(&pool, "me@example.com").await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::get()
        .uri("/api/me/overview")
        .insert_header((USER_ID_HEADER, me.to_string()))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body, serde_json::json!([]));
}

#[sqlx::test]
async fn get_overview_without_header_returns_401(pool: SqlitePool) {
    let app = common::app(&pool).await;

    let req = test::TestRequest::get().uri("/api/me/overview").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
