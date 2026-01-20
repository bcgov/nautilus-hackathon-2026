use rocket::{Build, Rocket};

pub mod index;

// The [launch] annotation generates a `main` method for this module
#[launch]
pub fn launch_webserver() -> Rocket<Build> {
    rocket::build().mount("/", routes![index::index])
}

