/* CRUD endpoints for the `Repository` model */
#[get("/")]
pub fn list_repositories() -> String {
    format!("All the repos!")
}

#[get("/<id>")]
pub fn get_repository(id: u64) -> String {
    format!("Repository id {id}")
}

#[post("/")]
pub fn create_repository() -> String {
    format!("Created repository!")
}

#[post("/<id>")]
pub fn update_repository(id: u64) -> String {
    format!("Updated repository id {id}")
}

#[delete("/<id>")]
pub fn delete_repository(id: u64) -> String {
    format!("Deleted repository id {id}")
}
