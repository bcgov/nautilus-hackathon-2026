use rocket::Route;
use rocket_dyn_templates::{context, Template};

pub fn routes() -> Vec<Route> {
    routes![
        index,
        repositories_page,
        repository_edit_page,
        pipelines_page,
        pipeline_edit_page
    ]
}

// Simple landing page with links to JSON routes.
#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/app/repositories")]
pub fn repositories_page() -> Template {
    Template::render("repositories", context! {})
}

#[get("/app/repositories/<repo_id>/edit")]
pub fn repository_edit_page(repo_id: u64) -> Template {
    Template::render("repository_edit", context! { repo_id })
}

#[get("/app/repositories/<repo_id>/pipelines")]
pub fn pipelines_page(repo_id: u64) -> Template {
    Template::render("pipelines", context! { repo_id })
}

#[get("/app/repositories/<repo_id>/pipelines/<pipeline_id>/edit")]
pub fn pipeline_edit_page(repo_id: u64, pipeline_id: u64) -> Template {
    Template::render("pipeline_edit", context! { repo_id, pipeline_id })
}
