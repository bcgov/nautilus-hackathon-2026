use rocket::{State, http::Status, serde::json::Json};
use sqlx::SqlitePool;

use crate::models::Repository;

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
pub fn get_repository(id: u64) -> String {
    format!("Repository id {id}")
}

#[post("/")]
pub fn create_repository() -> String {
    format!("Created repository!")
}

#[post("/<id>")]
pub fn update_repository(id: u64) -> String {
    format!("Updated repository id {id}")
}

#[delete("/<id>")]
pub fn delete_repository(id: u64) -> String {
    format!("Deleted repository id {id}")
}
