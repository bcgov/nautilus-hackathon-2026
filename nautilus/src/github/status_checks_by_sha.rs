// Function(s) to pull commit data from a defined github repo and branch

use reqwest::Error;
use reqwest::header::USER_AGENT;
use serde_json::Value;

use crate::environment::getenv;

pub async fn get_status_checks_by_sha(repo: &str, sha: &str) -> Result<Value, Error> {
    let mut path_string = String::from(repo);
    let api_path = format!("commits/{}/check-runs", sha);
    path_string.push_str(&api_path);

    let gh_key = getenv("GH_TOKEN", "");

    let client = reqwest::Client::new();
    let res = client
        .get(path_string)
        .header(USER_AGENT, "Nautilus-2026")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Authorization", format!("Bearer {gh_key}"))
        .send();
    let body = res.await.unwrap().text().await?;

    let json_val: Value = serde_json::from_str(&body).expect("badly formatted json");

    Ok(json_val)
}
