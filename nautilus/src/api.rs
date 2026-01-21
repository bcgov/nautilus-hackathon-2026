use sqlx::SqlitePool;
use rocket::{Build, Rocket, Route};


mod index;
mod repository;
mod pipeline;

pub fn get_routes() -> Vec<Route> {
    routes![
        repository::list_repositories,
        repository::get_repository,
        repository::create_repository,
        repository::update_repository,
        repository::delete_repository,
        pipeline::list_pipelines,
        pipeline::get_pipeline,
        pipeline::create_pipeline,
        pipeline::update_pipeline,
        pipeline::delete_pipeline,
        index::index,
        index::health,
        index::repositories,
        index::pipelines,
        index::deployments
    ]
}

// The [launch] annotation generates a `main` method for this module
#[launch]
pub fn launch_webserver() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index::index])
        .mount("/repository", get_routes())
}
