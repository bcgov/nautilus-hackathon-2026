
use sqlx::{SqlitePool, Error};

use crate::models::Pipeline;

pub async fn fetch(pool: &SqlitePool, id: u64) -> Result<Option<Pipeline>, Error> {
    sqlx::query_as::<_, Pipeline>(
        "select id, repository_id, name, branch_name, auto_deploy, created_at, updated_at from pipeline where id = ?",
    )
    .bind(id as i64)
    .fetch_optional(pool)
    .await
}

