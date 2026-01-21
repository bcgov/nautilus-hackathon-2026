use rocket::{Build, Rocket, Route};
use sqlx::SqlitePool;

mod index;
mod pipelines;
mod repositories;

pub fn get_routes() -> Vec<Route> {
    routes![
        repositories::list_repositories,
        repositories::get_repository,
        repositories::create_repository,
        repositories::update_repository,
        repositories::delete_repository,
        pipelines::list_pipelines,
        pipelines::get_pipeline,
        pipelines::create_pipeline,
        pipelines::update_pipeline,
        pipelines::delete_pipeline,
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
                index::deployments
            ],
        )
        .mount("/repositories", get_routes())
}
