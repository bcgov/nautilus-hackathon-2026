use std::env;

use anyhow::{Context, Result};
use sqlx::{Pool, Sqlite};

use crate::{github::clone, worker::deployment_service};

#[macro_use]
extern crate rocket;

mod api;
mod db;
mod github;
mod models;
mod worker;
mod environment;

#[rocket::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args: Vec<String> = env::args().collect();

    let database_url = "sqlite://nautilus.db";
    let pool = db::init_pool(&database_url).await?;

    if args.len() == 2 {
        if args[1] == "api" {
            run_api(pool).await?;
            return Ok(());
        } else if args[1] == "worker" {
            clone::by_sha("https://github.com/bcgov/nautilus-test-repo", "3262fce1fec6d98eeb3b65f96e9e53fb335917f7").ok();
            return Ok(());
        }
    }

    println!("");
    println!("Please pass an argument whether to start the api or the worker");
    println!("i.e. `cargo run -- api` or `cargo run -- worker`");
    println!("");
    Ok(())
}

async fn run_api(pool: Pool<Sqlite>) -> Result<()> {

    db::run_migrations(&pool).await?;

    api::launch_webserver(pool)
        .launch()
        .await
        .context("rocket server stopped unexpectedly")?;
    Ok(())
}

