use sqlx::{Error, SqlitePool};

use crate::models::{CreateDeployment, Deployment};

pub async fn create(pool: &SqlitePool, payload: CreateDeployment) -> Result<Deployment, Error> {
    sqlx::query_as::<_, Deployment>(
        "insert into deployment (pipeline_id, commit_sha, pr_id, status, created_at)
         values (?, ?, ?, 'in_progress', ?)
         returning id, pipeline_id, commit_sha, status, created_at, started_at, ended_at, pr_id",
    )
    .bind(payload.pipeline_id as i64)
    .bind(payload.commit_sha)
    .bind(payload.pr_id)
    .bind(chrono::Local::now().to_string())
    .fetch_one(pool)
    .await
}

pub async fn update_status(
    pool: &SqlitePool,
    deployment_id: i64,
    new_status: &str,
) -> Result<Deployment, Error> {
    sqlx::query_as::<_, Deployment>(
        "update deployment 
        set status = ? where id = ? 
        returning id, pipeline_id, commit_sha, status, created_at, started_at, ended_at, pr_id",
    )
    .bind(new_status)
    .bind(deployment_id)
    .fetch_one(pool)
    .await
}
