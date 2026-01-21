use rocket::{State, http::Status, serde::json::Json};
use sqlx::SqlitePool;

use crate::models::Pipeline;

/* CRUD endpoints for the `Repository` model */
#[get("/<repo_id>/pipeline")]
pub async fn list_pipelines(
    pool: &State<SqlitePool>,
    repo_id: u64,
) -> Result<Json<Vec<Pipeline>>, Status> {
    let rows = sqlx::query_as::<_, Pipeline>(
        format!("select id, repository_id, name, auto_deploy, created_at, updated_at 
                 from pipeline 
                 where repository_id = {repo_id} 
                 order by id asc").as_str()
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;
    Ok(Json(rows))
}

#[get("/<repo_id>/pipeline/<id>")]
pub fn get_pipeline(repo_id: u64, id: u64) -> String {
    format!("pipeline id {id} in repository id {repo_id}")
}

#[post("/<repo_id>/pipeline")]
pub fn create_pipeline(repo_id: u64) -> String {
    format!("Created pipeline in repo {repo_id}!")
}

#[post("/<repo_id>/pipeline/<id>")]
pub fn update_pipeline(repo_id: u64, id: u64) -> String {
    format!("Updated pipeline {id} in repository id {repo_id}")
}

#[delete("/<repo_id>/pipeline/<id>")]
pub fn delete_pipeline(repo_id: u64, id: u64) -> String {
    format!("Deleted pipeline {id} in repository id {repo_id}")
}
