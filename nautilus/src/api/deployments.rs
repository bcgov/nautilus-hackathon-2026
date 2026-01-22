use std::thread;

use rocket::{http::Status, serde::json::Json, State};
use sqlx::SqlitePool;

use crate::github::github_api;
use crate::github::github_api::PrCommitData;
use crate::models::{Deployment, PipelineRepo};
use crate::worker::deployment_service;

#[derive(rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DeploymentsView {
    pub deployed: Vec<Deployment>,
    pub pending: Vec<PrCommitData>,
}

pub async fn load_deployments(
    pool: &State<SqlitePool>,
    repo_id: u64,
    id: u64,
) -> Result<DeploymentsView, Status> {
    // 1) Look up the repo URL + pipeline branch for this pipeline.
    let row = sqlx::query_as::<_, PipelineRepo>(
        "select repository.url as repo_url, pipeline.branch_name as branch_name
         from pipeline
         join repository on repository.id = pipeline.repository_id
         where pipeline.repository_id = ? and pipeline.id = ?",
    )
    .bind(repo_id as i64)
    .bind(id as i64)
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    let row = match row {
        Some(value) => value,
        None => return Err(Status::NotFound),
    };

    // 2) Convert the repo URL to a GitHub API base URL.
    let api_base = github_api::repo_api_base(&row.repo_url).ok_or(Status::BadRequest)?;

    // 3) Load successful deployments from the database.
    let deployed = sqlx::query_as::<_, Deployment>(
        "select id, pipeline_id, commit_sha, status, created_at, started_at, ended_at, pr_id
         from deployment
         where pipeline_id = ? and status = 'successful'
         order by created_at desc",
    )
    .bind(id as i64)
    .fetch_all(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    // 4) Get the last deployed SHA for this pipeline.
    let last_sha = sqlx::query_scalar::<_, String>(
        "select commit_sha from deployment
         where pipeline_id = ? and commit_sha is not null
         order by created_at desc
         limit 1",
    )
    .bind(id as i64)
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    // 5) Pull merged PRs from GitHub after the last deployed SHA.
    let pending = github_api::poll_merge_commits(&api_base, &row.branch_name, last_sha.as_deref())
        .await;
    Ok(DeploymentsView { deployed, pending })
}

#[get("/<repo_id>/pipelines/<id>/deployments")]
pub async fn deployments(
    pool: &State<SqlitePool>,
    repo_id: u64,
    id: u64,
) -> Result<Json<DeploymentsView>, Status> {
    let view = load_deployments(pool, repo_id, id).await?;
    Ok(Json(view))
}

#[post("/<repo_id>/pipelines/<pipeline_id>/deployments/<sha>")]
pub fn create_deployments(
    pool: &State<SqlitePool>,
    repo_id: u64,
    pipeline_id: u64,
    sha: String
) -> Result<String, Status> {
    let text = format!("Deploying repo {} for pipeline {} and sha {}", repo_id, pipeline_id,sha);
    let sql_pool = pool.inner().clone();

    thread::spawn(async move || {
        deployment_service::deploy(&sql_pool, pipeline_id, sha.as_str()).await.ok();
    });
    
    Ok(text)
}
