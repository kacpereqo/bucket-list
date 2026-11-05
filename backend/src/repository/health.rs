use sqlx::SqlitePool;

pub async fn get_unixepoch(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar!("SELECT unixepoch()")
        .fetch_one(pool)
        .await
}

pub async fn get_unixepoch_as_json_string(pool: &SqlitePool) -> Result<String, sqlx::Error> {
    sqlx::query_scalar!(r#"SELECT JSON(json_object('epoch', unixepoch()))"#)
        .fetch_one(pool)
        .await?
        .ok_or(sqlx::Error::BeginFailed)
}
