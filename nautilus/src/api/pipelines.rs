use rocket::{http::Status, serde::json::Json, State};
use sqlx::SqlitePool;

use crate::models::{CreatePipeline, Pipeline, UpdatePipeline};

/* CRUD endpoints for pipelines scoped to a repository */
#[get("/<repo_id>/pipelines")]
pub async fn list_pipelines(
    pool: &State<SqlitePool>,
    repo_id: u64,
) -> Result<Json<Vec<Pipeline>>, Status> {
    let rows = sqlx::query_as::<_, Pipeline>(
        "select id, repository_id, name, auto_deploy, created_at, updated_at
         from pipeline
         where repository_id = ?
         order by id asc",
    )
    .bind(repo_id as i64)
    .fetch_all(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;
    Ok(Json(rows))
}

#[get("/<repo_id>/pipelines/<id>")]
pub async fn get_pipeline(
    pool: &State<SqlitePool>,
    repo_id: u64,
    id: u64,
) -> Result<Json<Pipeline>, Status> {
    let row = sqlx::query_as::<_, Pipeline>(
        "select id, repository_id, name, auto_deploy, created_at, updated_at
         from pipeline
         where repository_id = ? and id = ?",
    )
    .bind(repo_id as i64)
    .bind(id as i64 )
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    match row {
        Some(pipeline) => Ok(Json(pipeline)),
        None => Err(Status::NotFound),
    }
}

#[post("/<repo_id>/pipelines", data = "<payload>")]
pub async fn create_pipeline(
    pool: &State<SqlitePool>,
    repo_id: u64,
    payload: Json<CreatePipeline>,
) -> Result<Json<Pipeline>, Status> {
    let created = sqlx::query_as::<_, Pipeline>(
        "insert into pipeline (repository_id, name, auto_deploy, created_at, updated_at)
         values (?, ?, ?, ?, ?)
         returning id, repository_id, name, auto_deploy, created_at, updated_at",
    )
    .bind(repo_id as i64)
    .bind(&payload.name)
    .bind(payload.auto_deploy)
    .bind(&payload.created_at)
    .bind(&payload.updated_at)
    .fetch_one(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    Ok(Json(created))
}

#[put("/<repo_id>/pipelines/<id>", data = "<payload>")]
pub async fn update_pipeline(
    pool: &State<SqlitePool>,
    repo_id: u64,
    id: u64,
    payload: Json<UpdatePipeline>,
) -> Result<Json<Pipeline>, Status> {
    let updated = sqlx::query_as::<_, Pipeline>(
        "update pipeline
         set name = ?, auto_deploy = ?, updated_at = ?
         where repository_id = ? and id = ?
         returning id, repository_id, name, auto_deploy, created_at, updated_at",
    )
    .bind(&payload.name)
    .bind(payload.auto_deploy)
    .bind(&payload.updated_at)
    .bind(repo_id as i64)
    .bind(id as i64)
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    match updated {
        Some(pipeline) => Ok(Json(pipeline)),
        None => Err(Status::NotFound),
    }
}

#[delete("/<repo_id>/pipelines/<id>")]
pub async fn delete_pipeline(
    pool: &State<SqlitePool>,
    repo_id: u64,
    id: u64,
) -> Result<Status, Status> {
    let result = sqlx::query("delete from pipeline where repository_id = ? and id = ?")
        .bind(repo_id as i64)
        .bind(id as i64)
        .execute(pool.inner())
        .await
        .map_err(|_| Status::InternalServerError)?;

    if result.rows_affected() == 0 {
        Err(Status::NotFound)
    } else {
        Ok(Status::NoContent)
    }
}
