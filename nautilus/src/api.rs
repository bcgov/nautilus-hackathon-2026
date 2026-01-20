use rocket::{Build, Rocket};

pub mod index;

pub fn launch_webserver() -> Rocket<Build> {
    rocket::build().mount("/", routes![index::index])
}