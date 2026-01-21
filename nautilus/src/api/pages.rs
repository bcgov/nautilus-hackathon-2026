use rocket::{http::Status, Route, State};
use rocket_dyn_templates::{context, Template};
use sqlx::SqlitePool;

use crate::api::deployments;

pub fn routes() -> Vec<Route> {
    routes![
        index,
        repositories_page,
        repository_edit_page,
        pipelines_page,
        pipeline_edit_page,
        deployments_page
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

#[get("/app/repositories/<repo_id>/pipelines/<pipeline_id>/deployments")]
pub async fn deployments_page(
    pool: &State<SqlitePool>,
    repo_id: u64,
    pipeline_id: u64,
) -> Result<Template, Status> {
    let (view, error_message) = match deployments::load_deployments(pool, repo_id, pipeline_id).await {
        Ok(view) => (view, None),
        Err(status) if status.code == 404 => (
            deployments::DeploymentsView {
                deployed: Vec::new(),
                pending: Vec::new(),
            },
            Some("Pipeline not found.".to_string()),
        ),
        Err(status) if status.code == 400 => (
            deployments::DeploymentsView {
                deployed: Vec::new(),
                pending: Vec::new(),
            },
            Some("Repository URL is invalid for GitHub.".to_string()),
        ),
        Err(_) => (
            deployments::DeploymentsView {
                deployed: Vec::new(),
                pending: Vec::new(),
            },
            Some("Failed to load deployments.".to_string()),
        ),
    };
    Ok(Template::render(
        "deployments",
        context! {
            repo_id,
            pipeline_id,
            deployed: view.deployed,
            pending: view.pending,
            error_message
        },
    ))
}
