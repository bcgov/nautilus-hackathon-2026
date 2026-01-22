use sqlx::SqlitePool;

use crate::{
    db::{deployment, pipeline, repository}, github::clone, models::CreateDeployment, worker::shell
};

pub async fn deploy(pool: &SqlitePool, pipeline_id: u64, commit_sha: &str) -> Result<(), String> {
    // 0. create deployment object
    let pipeline = match pipeline::fetch(pool, pipeline_id).await {
        Ok(Some(p)) => p,
        Ok(None) => return Err("No Pipeline Found".to_string()),
        Err(e) => return Err(e.to_string())
    };

    let repo = match repository::fetch(pool, pipeline.id).await {
        Ok(Some(r)) => r,
        Ok(None) => return Err("No Repository Found".to_string()),
        Err(e) => return Err(e.to_string())
    };

    let deployment = deployment::create(
        pool,
        CreateDeployment {
            pipeline_id: pipeline.id,
            commit_sha: commit_sha.to_string(),
            pr_id: "0".to_string(),
        },
    ).await.unwrap();

    // 1. pull repo & commit (shallow)
    let cloned_dir = clone::by_sha(&repo.url, commit_sha)?;
    let nautilus_file = cloned_dir + "/nautilus.sh";

    // 2. execute nautilus.sh from that path
    shell::run_shell_file(&nautilus_file).await.unwrap();

    Ok(())
}
