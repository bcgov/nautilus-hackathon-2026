
/* CRUD endpoints for the `Repository` model */
#[get("/<repo_id>/pipeline")]
pub fn list_pipelines(repo_id: u64) -> String {
    format!("All the pipelines for repo id {repo_id}!")
}

#[get("/<repo_id>/pipeline/<id>")]
pub fn get_pipeline(repo_id: u64, id: u64) -> String {
    format!("pipeline id {id} in repository id {repo_id}")
}

#[post("/<repo_id>/pipeline")]
pub fn create_pipeline(repo_id: u64) -> String {
    format!("Created pipeline in repo {repo_id}!")
}

#[post("/<repo_id>/pipeline/<id>")]
pub fn update_pipeline(repo_id: u64, id: u64) -> String {
    format!("Updated pipeline {id} in repository id {repo_id}")
}

#[delete("/<repo_id>/pipeline/<id>")]
pub fn delete_pipeline(repo_id: u64, id: u64) -> String {
    format!("Deleted pipeline {id} in repository id {repo_id}")
}
