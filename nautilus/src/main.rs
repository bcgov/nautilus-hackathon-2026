#[macro_use] extern crate rocket;

mod api;

#[launch]
fn rocket() -> _ {
    api::launch_webserver()
}
