use rocket::{http::Status, serde::json::Json, State};
use sqlx::SqlitePool;

use crate::models::{CreateRepository, Repository, UpdateRepository};

/* CRUD endpoints for the `Repository` model */
#[get("/")]
pub async fn list_repositories(pool: &State<SqlitePool>) -> Result<Json<Vec<Repository>>, Status>  {
        // `query_as` maps rows into the Repository struct.
    let rows = sqlx::query_as::<_, Repository>(
        "select id, name, url, created_at, updated_at from repository order by id asc",
    )
    .fetch_all(pool.inner()) // run the query using the shared pool
    .await
    .map_err(|_| Status::InternalServerError)?;
    Ok(Json(rows))
}

#[get("/<id>")]
pub async fn get_repository(
    pool: &State<SqlitePool>,
    id: u64,
) -> Result<Json<Repository>, Status> {
    let row = sqlx::query_as::<_, Repository>(
        "select id, name, url, created_at, updated_at from repository where id = ?",
    )
    .bind(id as i64)
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    match row {
        Some(repo) => Ok(Json(repo)),
        None => Err(Status::NotFound),
    }
}

#[post("/", data = "<payload>")]
pub async fn create_repository(
    pool: &State<SqlitePool>,
    payload: Json<CreateRepository>,
) -> Result<Json<Repository>, Status> {
    let created = sqlx::query_as::<_, Repository>(
        "insert into repository (name, url, created_at, updated_at)
         values (?, ?, ?, ?)
         returning id, name, url, created_at, updated_at",
    )
    .bind(&payload.name)
    .bind(&payload.url)
    .bind(&payload.created_at)
    .bind(&payload.updated_at)
    .fetch_one(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    Ok(Json(created))
}

#[put("/<id>", data = "<payload>")]
pub async fn update_repository(
    pool: &State<SqlitePool>,
    id: u64,
    payload: Json<UpdateRepository>,
) -> Result<Json<Repository>, Status> {
    let updated = sqlx::query_as::<_, Repository>(
        "update repository
         set name = ?, url = ?, updated_at = ?
         where id = ?
         returning id, name, url, created_at, updated_at",
    )
    .bind(&payload.name)
    .bind(&payload.url)
    .bind(&payload.updated_at)
    .bind(id as i64)
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    match updated {
        Some(repo) => Ok(Json(repo)),
        None => Err(Status::NotFound),
    }
}

#[delete("/<id>")]
pub async fn delete_repository(
    pool: &State<SqlitePool>,
    id: u64,
) -> Result<Status, Status> {
    let result = sqlx::query("delete from repository where id = ?")
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
