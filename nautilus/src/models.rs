use rocket::serde::Serialize; // serde => Serialize/Deserialize
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

#[derive(FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Pipeline {
    pub id: i64,
    pub repository_id: i64,
    pub name: String,
    pub auto_deploy: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(FromRow, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Deployment {
    pub id: i64,
    pub pipeline_id: i64,
    pub commit_sha: Option<String>, // Option<T> => T or None
    pub status: String,
    pub created_at: String,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub pr_id: Option<i64>,
}
