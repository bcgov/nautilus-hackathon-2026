use sqlx::{SqlitePool, Error};

use crate::models::Repository;

pub async fn get_repository(pool: &SqlitePool, id: u64) -> Result<Option<Repository>, Error> {
    sqlx::query_as::<_, Repository>(
        "select id, name, url, created_at, updated_at from repository where id = ?",
    )
    .bind(id as i64)
    .fetch_optional(pool)
    .await
}

