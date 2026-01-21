// Function(s) to pull commit data from a defined github repo and branch


use serde_json::Value;
use serde_json;
use reqwest::Error;
use reqwest::header::{USER_AGENT};

#[derive(Debug)]
pub struct PrCommitData {
    title: String,
    merged_at: String,
    merge_commit_sha: String
}

impl PrCommitData {
  pub fn new(title: &str, merged_at: &str, merge_commit_sha: &str) -> PrCommitData {
      PrCommitData {
          title: title.to_string(),
          merged_at: merged_at.to_string(),
          merge_commit_sha: merge_commit_sha.to_string()
      }
  }
  pub fn get_merge_commit_sha(&self) -> &str {
      return &self.merge_commit_sha;
  }
}

#[tokio::main]
pub async fn poll_merge_commits(repo: &str, branch: &str) -> Result<Vec<PrCommitData>, Error> {

  let mut path_string = String::from(repo);
  let query_params = format!("?state=closed&base={}&per_page=3&sort=updated&direction=desc", branch);
  path_string.push_str(&query_params);

  let client = reqwest::Client::new();
  let res = client.get(path_string).header(USER_AGENT, "Nautilus-2026").header("Accept", "application/vnd.github+json").header("X-GitHub-Api-Version", "2022-11-28").send();
  let body = res.await.unwrap().text().await?;

  let json_val: Vec<Value> = serde_json::from_str(&body).expect("badly formatted json");

  // Simulate last deployed sha
  let saved_sha: &str = "8534eac40f98f8680c216293e7c8c943d24142bb";

  let mut commits_array: Vec<PrCommitData> = Vec::new();

  println!("\n\nNautilus is displaying undeployed commits for repo {}:\n", repo);
  for i in &json_val {
    if i["merge_commit_sha"] == saved_sha {
      break;
    }
    let data = PrCommitData::new(&i["title"].to_string(), &i["merged_at"].to_string(), &i["merge_commit_sha"].to_string());

    println!("Title: {}\nMerged: {}\nMerge Commit Sha: {}\n", i["title"], i["merged_at"], i["merge_commit_sha"]);
    commits_array.push(data);
  }
  Ok(commits_array)
}

