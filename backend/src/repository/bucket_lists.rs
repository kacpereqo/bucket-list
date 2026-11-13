use sqlx::SqlitePool;

use crate::models::{
    bucket_lists::{
        BucketList, BucketListCreate, BucketListUpdate, BucketListWithLists, TodoListWithItems,
    },
    todo_items::TodoItem,
    todo_lists::TodoList,
};

pub async fn get_accessible_by_user_id(
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

pub async fn get_accessible_with_lists_and_items_by_user_id(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Vec<BucketListWithLists>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT
            b.id          AS "bucket_id!",
            b.owner_id    AS "bucket_owner_id!",
            b.name        AS "bucket_name!",
            b.created_at  AS "bucket_created_at!",
            l.id          AS "list_id?",
            l.name        AS "list_name?",
            l.created_at  AS "list_created_at?",
            i.id          AS "item_id?",
            i.title       AS "item_title?",
            i.description AS "item_description?",
            i.is_done     AS "item_is_done?: bool",
            i.created_at  AS "item_created_at?"
        FROM bucket_lists b
        JOIN shared_bucket_lists s ON s.bucket_list_id = b.id
        LEFT JOIN todo_lists l ON l.bucket_list_id = b.id
        LEFT JOIN todo_items i ON i.todo_list_id = l.id
        WHERE s.user_id = ?
        ORDER BY b.id, l.id, i.id
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    let mut buckets: Vec<BucketListWithLists> = Vec::new();

    for row in rows {
        if buckets.last().map(|b| b.bucket.id) != Some(row.bucket_id) {
            buckets.push(BucketListWithLists {
                bucket: BucketList {
                    id: row.bucket_id,
                    owner_id: row.bucket_owner_id,
                    name: row.bucket_name,
                    created_at: row.bucket_created_at,
                },
                todo_lists: Vec::new(),
            });
        }
        let bucket = buckets.last_mut().expect("bucket was just pushed");

        let (Some(list_id), Some(list_name), Some(list_created_at)) =
            (row.list_id, row.list_name, row.list_created_at)
        else {
            continue;
        };

        if bucket.todo_lists.last().map(|l| l.list.id) != Some(list_id) {
            bucket.todo_lists.push(TodoListWithItems {
                list: TodoList {
                    id: list_id,
                    bucket_list_id: row.bucket_id,
                    name: list_name,
                    created_at: list_created_at,
                },
                items: Vec::new(),
            });
        }
        let list = bucket.todo_lists.last_mut().expect("list was just pushed");

        let (Some(item_id), Some(title), Some(is_done), Some(item_created_at)) = (
            row.item_id,
            row.item_title,
            row.item_is_done,
            row.item_created_at,
        ) else {
            continue;
        };

        list.items.push(TodoItem {
            id: item_id,
            todo_list_id: list_id,
            title,
            description: row.item_description,
            is_done,
            created_at: item_created_at,
        });
    }

    Ok(buckets)
}

pub async fn get_accessible_with_lists_and_items_by_user_id_as_json_string(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<String, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        SELECT json_group_array(json(bucket)) AS "json!: String"
        FROM (
            SELECT json_object(
                'id', b.id,
                'owner_id', b.owner_id,
                'name', b.name,
                'created_at', b.created_at,
                'todo_lists', json((
                    SELECT json_group_array(json(list))
                    FROM (
                        SELECT json_object(
                            'id', l.id,
                            'bucket_list_id', l.bucket_list_id,
                            'name', l.name,
                            'created_at', l.created_at,
                            'items', json((
                                SELECT json_group_array(json(item))
                                FROM (
                                    SELECT json_object(
                                        'id', i.id,
                                        'todo_list_id', i.todo_list_id,
                                        'title', i.title,
                                        'description', i.description,
                                        'is_done', json(CASE WHEN i.is_done THEN 'true' ELSE 'false' END),
                                        'created_at', i.created_at
                                    ) AS item
                                    FROM todo_items i
                                    WHERE i.todo_list_id = l.id
                                    ORDER BY i.id
                                )
                            ))
                        ) AS list
                        FROM todo_lists l
                        WHERE l.bucket_list_id = b.id
                        ORDER BY l.id
                    )
                ))
            ) AS bucket
            FROM bucket_lists b
            JOIN shared_bucket_lists s ON s.bucket_list_id = b.id
            WHERE s.user_id = ?
            ORDER BY b.id
        )
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn create(
    pool: &SqlitePool,
    owner_id: i64,
    bucket_list: BucketListCreate,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO bucket_lists (owner_id, name) VALUES (?, ?)",
        owner_id,
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
