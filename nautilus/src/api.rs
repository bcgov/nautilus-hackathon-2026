use rocket::{Build, Rocket, Route};
use sqlx::SqlitePool;

mod index;
mod pipeline;
mod repository;

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
    ]
}

pub fn launch_webserver(pool: SqlitePool) -> Rocket<Build> {
    rocket::build()
        .manage(pool)
        .mount(
            "/",
            routes![
                index::index,
                index::health,
                index::repositories,
                index::pipelines,
                index::deployments
            ],
        )
        .mount("/repository", get_routes())
}
