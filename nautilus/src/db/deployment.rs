
use sqlx::{SqlitePool, Error};

use crate::models::{Deployment, CreateDeployment};

pub async fn create(pool: &SqlitePool, payload: CreateDeployment) -> Result<Deployment, Error> {
    sqlx::query_as::<_, Deployment>(
        "insert into deployment (pipeline_id, commit_sha, pr_id)
         values (?, ?, ?)
         returning id, pipeline_id, commit_sha, status, created_at, started_at, ended_at, pr_id",
    )
    .bind(payload.pipeline_id as i64)
    .bind(payload.commit_sha)
    .bind(payload.pr_id)
    .fetch_one(pool)
    .await
}

