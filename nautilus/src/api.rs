use rocket::{Build, Rocket};
use sqlx::SqlitePool;

pub mod index;

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
}

