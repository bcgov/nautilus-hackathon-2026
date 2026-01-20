// Function(s) to pull commit data from a defined github repo and branch


use serde_json::Value;
use serde_json;
use reqwest::Error;
use reqwest::header::{USER_AGENT};

#[tokio::main]
pub async fn poll_merge_commits(repo: &str, branch: &str) -> Result<(), Error> {

  let mut path_string = String::from(repo);
  let query_params = format!("?state=closed&base={}&per_page=3&sort=updated&direction=desc", branch);
  path_string.push_str(&query_params);

  let client = reqwest::Client::new();
  let res = client.get(path_string).header(USER_AGENT, "Nautilus-2026").header("Accept", "application/vnd.github+json").header("X-GitHub-Api-Version", "2022-11-28").send();
  let body = res.await.unwrap().text().await?;

  let json_val: Vec<Value> = serde_json::from_str(&body).expect("badly formatted json");

  println!("\n\nNautilus is displaying undeployed commits for repo {}:\n", repo);
  for i in json_val {
    println!("Title: {}\nMerged: {}\nMerge Commit Sha: {}\n", i["title"], i["merged_at"], i["merge_commit_sha"]);
  }
  Ok(())
}

