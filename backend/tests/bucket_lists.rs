mod common;

use actix_web::{
    http::{header::ContentType, StatusCode},
    test,
};
use backend::auth::USER_ID_HEADER;
use sqlx::SqlitePool;

async fn name_of(pool: &SqlitePool, id: i64) -> String {
    sqlx::query_scalar("SELECT name FROM bucket_lists WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[sqlx::test]
async fn create_sets_owner_from_header(pool: SqlitePool) {
    let me = common::insert_user(&pool, "me@example.com").await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::post()
        .uri("/api/buckets")
        .insert_header((USER_ID_HEADER, me.to_string()))
        .insert_header(ContentType::json())
        .set_payload(r#"{"name": "Trips"}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);
    let new_id: i64 = test::read_body_json(resp).await;

    let (owner_id, name): (i64, String) =
        sqlx::query_as("SELECT owner_id, name FROM bucket_lists WHERE id = ?")
            .bind(new_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(owner_id, me);
    assert_eq!(name, "Trips");

    // the add_owner_to_shared trigger makes it visible in /me/buckets
    let shared: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM shared_bucket_lists WHERE user_id = ? AND bucket_list_id = ?",
    )
    .bind(me)
    .bind(new_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(shared, 1);
}

#[sqlx::test]
async fn create_without_header_returns_401(pool: SqlitePool) {
    let app = common::app(&pool).await;

    let req = test::TestRequest::post()
        .uri("/api/buckets")
        .insert_header(ContentType::json())
        .set_payload(r#"{"name": "Trips"}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn create_without_name_returns_400(pool: SqlitePool) {
    let me = common::insert_user(&pool, "me@example.com").await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::post()
        .uri("/api/buckets")
        .insert_header((USER_ID_HEADER, me.to_string()))
        .insert_header(ContentType::json())
        .set_payload(r#"{}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test]
async fn owner_can_rename(pool: SqlitePool) {
    let me = common::insert_user(&pool, "me@example.com").await;
    let id = common::insert_bucket_list(&pool, me).await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::put()
        .uri(&format!("/api/buckets/{id}"))
        .insert_header((USER_ID_HEADER, me.to_string()))
        .insert_header(ContentType::json())
        .set_payload(r#"{"name": "Renamed"}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    assert_eq!(name_of(&pool, id).await, "Renamed");
}

#[sqlx::test]
async fn non_owner_cannot_rename(pool: SqlitePool) {
    let owner = common::insert_user(&pool, "owner@example.com").await;
    let other = common::insert_user(&pool, "other@example.com").await;
    let id = common::insert_bucket_list(&pool, owner).await;
    let original = name_of(&pool, id).await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::put()
        .uri(&format!("/api/buckets/{id}"))
        .insert_header((USER_ID_HEADER, other.to_string()))
        .insert_header(ContentType::json())
        .set_payload(r#"{"name": "Hacked"}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    assert_eq!(name_of(&pool, id).await, original);
}

#[sqlx::test]
async fn rename_missing_list_returns_404(pool: SqlitePool) {
    let me = common::insert_user(&pool, "me@example.com").await;
    let app = common::app(&pool).await;

    let req = test::TestRequest::put()
        .uri("/api/buckets/999")
        .insert_header((USER_ID_HEADER, me.to_string()))
        .insert_header(ContentType::json())
        .set_payload(r#"{"name": "Renamed"}"#)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn tree_json_contains_accessible_buckets_with_lists_and_items(pool: SqlitePool) {
    let me = common::insert_user(&pool, "me@example.com").await;
    let other = common::insert_user(&pool, "other@example.com").await;

    let mine = common::insert_bucket_list(&pool, me).await;
    let list = common::insert_todo_list(&pool, mine).await;
    let item = common::insert_todo_item(&pool, list).await;
    let empty_list = common::insert_todo_list(&pool, mine).await;
    sqlx::query("UPDATE todo_items SET is_done = TRUE, description = 'desc' WHERE id = ?")
        .bind(item)
        .execute(&pool)
        .await
        .unwrap();

    let empty_bucket = common::insert_bucket_list(&pool, me).await;
    let _not_mine = common::insert_bucket_list(&pool, other).await;

    let json = backend::repository::bucket_lists::get_accessible_with_lists_and_items_by_user_id_as_json_string(
        &pool, me,
    )
    .await
    .unwrap();
    let mut tree: serde_json::Value = serde_json::from_str(&json).unwrap();

    // created_at depends on the clock, drop it before comparing
    fn strip_created_at(value: &mut serde_json::Value) {
        match value {
            serde_json::Value::Object(map) => {
                map.remove("created_at");
                map.values_mut().for_each(strip_created_at);
            }
            serde_json::Value::Array(values) => values.iter_mut().for_each(strip_created_at),
            _ => {}
        }
    }
    strip_created_at(&mut tree);

    assert_eq!(
        tree,
        serde_json::json!([
            {
                "id": mine, "owner_id": me, "name": "Test bucket list",
                "todo_lists": [
                    {
                        "id": list, "bucket_list_id": mine, "name": "Test todo list",
                        "items": [
                            {
                                "id": item, "todo_list_id": list, "title": "Test item",
                                "description": "desc", "is_done": true
                            }
                        ]
                    },
                    { "id": empty_list, "bucket_list_id": mine, "name": "Test todo list", "items": [] }
                ]
            },
            { "id": empty_bucket, "owner_id": me, "name": "Test bucket list", "todo_lists": [] }
        ])
    );
}

#[sqlx::test]
async fn tree_json_is_empty_array_for_user_without_buckets(pool: SqlitePool) {
    let me = common::insert_user(&pool, "me@example.com").await;

    let json = backend::repository::bucket_lists::get_accessible_with_lists_and_items_by_user_id_as_json_string(
        &pool, me,
    )
    .await
    .unwrap();

    assert_eq!(json, "[]");
}

#[sqlx::test]
async fn tree_typed_matches_tree_json_string(pool: SqlitePool) {
    use backend::repository::bucket_lists::{
        get_accessible_with_lists_and_items_by_user_id,
        get_accessible_with_lists_and_items_by_user_id_as_json_string,
    };

    let me = common::insert_user(&pool, "me@example.com").await;
    let other = common::insert_user(&pool, "other@example.com").await;

    // own bucket: list with two items (one done, one with NULL description) + empty list
    let mine = common::insert_bucket_list(&pool, me).await;
    let list = common::insert_todo_list(&pool, mine).await;
    let done = common::insert_todo_item(&pool, list).await;
    common::insert_todo_item(&pool, list).await;
    common::insert_todo_list(&pool, mine).await;
    sqlx::query("UPDATE todo_items SET is_done = TRUE, description = 'desc' WHERE id = ?")
        .bind(done)
        .execute(&pool)
        .await
        .unwrap();

    // empty own bucket, bucket shared with me, and a bucket I can't see
    common::insert_bucket_list(&pool, me).await;
    let shared = common::insert_bucket_list(&pool, other).await;
    let shared_list = common::insert_todo_list(&pool, shared).await;
    common::insert_todo_item(&pool, shared_list).await;
    sqlx::query("INSERT INTO shared_bucket_lists (user_id, bucket_list_id) VALUES (?, ?)")
        .bind(me)
        .bind(shared)
        .execute(&pool)
        .await
        .unwrap();
    let hidden = common::insert_bucket_list(&pool, other).await;
    common::insert_todo_list(&pool, hidden).await;

    let typed = get_accessible_with_lists_and_items_by_user_id(&pool, me)
        .await
        .unwrap();
    let from_sql: serde_json::Value = serde_json::from_str(
        &get_accessible_with_lists_and_items_by_user_id_as_json_string(&pool, me)
            .await
            .unwrap(),
    )
    .unwrap();

    assert_eq!(typed.len(), 3);
    assert!(typed.iter().all(|b| b.bucket.id != hidden));
    assert_eq!(serde_json::to_value(&typed).unwrap(), from_sql);
}

#[sqlx::test]
async fn tree_typed_is_empty_for_user_without_buckets(pool: SqlitePool) {
    let me = common::insert_user(&pool, "me@example.com").await;

    let typed = backend::repository::bucket_lists::get_accessible_with_lists_and_items_by_user_id(
        &pool, me,
    )
    .await
    .unwrap();

    assert!(typed.is_empty());
}
