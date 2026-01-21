// Function(s) to pull commit data from a defined github repo and branch


use serde_json::Value;
use serde_json;
use reqwest::Error;
use reqwest::header::{USER_AGENT};

pub async fn get_merge_commits(repo: &str, branch: &str) -> Result<Vec<Value>, Error> {

  let mut path_string = String::from(repo);
  let query_params = format!("?state=closed&base={}&per_page=20&sort=updated&direction=desc", branch);
  path_string.push_str("pulls");
  path_string.push_str(&query_params);

  let client = reqwest::Client::new();
  let res = client.get(path_string).header(USER_AGENT, "Nautilus-2026").header("Accept", "application/vnd.github+json").header("X-GitHub-Api-Version", "2022-11-28").send();
  let body = res.await.unwrap().text().await?;

  let json_val: Vec<Value> = serde_json::from_str(&body).expect("badly formatted json");

  Ok(json_val)
}

