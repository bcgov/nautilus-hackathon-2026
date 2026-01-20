use std::env;

use anyhow::{Context, Result};

#[macro_use]
extern crate rocket;

mod api;
mod db;
mod models;

#[rocket::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        if args[1] == "api" {
            run_api().await?;
            return Ok(());
        } else if args[1] == "worker" {
            println!("Worker not implemented yet");
            return Ok(());
        }
    }

    println!("");
    println!("Please pass an argument whether to start the api or the worker");
    println!("i.e. `cargo run -- api` or `cargo run -- worker`");
    println!("");
    Ok(())
}

async fn run_api() -> Result<()> {
    let database_url = "sqlite://nautilus.db";
    let pool = db::init_pool(&database_url).await?;
    db::run_migrations(&pool).await?;

    api::launch_webserver(pool)
        .launch()
        .await
        .context("rocket server stopped unexpectedly")?;
    Ok(())
}