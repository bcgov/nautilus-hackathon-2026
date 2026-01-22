use sqlx::SqlitePool;

use crate::{
    db::deployment::create_deployment,
    models::{CreateDeployment},
};

pub fn deploy(pool: &SqlitePool, pipeline_id: u64, commit_sha: &str) {
    // 0. create deployment object
    let deployment = create_deployment(
        pool,
        CreateDeployment {
            pipeline_id: (),
            commit_sha: (),
            pr_id: (),
        },
    );

    // 1. pull repo & commit (shallow)

    // 2.
}
