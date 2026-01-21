use rocket::serde::{Deserialize, Serialize}; // serde => Serialize/Deserialize
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
#[serde(crate = "rocket::serde")] // needed for rocket to serialize the struct
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateRepository {
    pub name: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateRepository {
    pub name: String,
    pub url: String,
    pub updated_at: String,
}

#[derive(FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Pipeline {
    pub id: i64,
    pub repository_id: i64,
    pub name: String,
    pub branch_name: String,
    pub auto_deploy: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreatePipeline {
    pub name: String,
    pub branch_name: String,
    pub auto_deploy: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdatePipeline {
    pub name: String,
    pub branch_name: String,
    pub auto_deploy: i64,
    pub updated_at: String,
}

#[derive(FromRow)]
pub struct PipelineRepo {
    pub repo_url: String,
    pub branch_name: String,
}

#[derive(FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Deployment {
    pub id: i64,
    pub pipeline_id: i64,
    pub commit_sha: Option<String>,
    pub status: String,
    pub created_at: String,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub pr_id: Option<i64>,
}