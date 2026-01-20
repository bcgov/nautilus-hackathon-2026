
use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::SqlitePool;

use crate::models::{Deployment, Pipeline, Repository};

// Simple landing page with links to JSON routes.
#[get("/")]
pub fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../../static/index.html"))
}

// Lightweight health check for uptime probes.
#[get("/health")]
pub fn health() -> &'static str {
    "ok"
}

#[get("/repositories")]
pub async fn repositories(pool: &State<SqlitePool>) -> Result<Json<Vec<Repository>>, Status> {
    // `query_as` maps rows into the Repository struct.
    let rows = sqlx::query_as::<_, Repository>(
        "select id, name, url, created_at, updated_at from repository order by id asc",
    )
    .fetch_all(pool.inner()) // run the query using the shared pool
    .await
    .map_err(|_| Status::InternalServerError)?;
    Ok(Json(rows))
}

#[get("/pipelines")]
pub async fn pipelines(pool: &State<SqlitePool>) -> Result<Json<Vec<Pipeline>>, Status> {
    let rows = sqlx::query_as::<_, Pipeline>(
        "select id, repository_id, name, auto_deploy, created_at, updated_at from pipeline order by id asc",
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;
    Ok(Json(rows))
}

#[get("/deployments")]
pub async fn deployments(pool: &State<SqlitePool>) -> Result<Json<Vec<Deployment>>, Status> {
    let rows = sqlx::query_as::<_, Deployment>(
        "select id, pipeline_id, commit_sha, status, created_at, started_at, ended_at, pr_id from deployment order by id asc",
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;
    Ok(Json(rows))
}


