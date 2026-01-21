// Function(s) to pull commit data from a defined github repo and branch
use crate::github::merge_commits;
use crate::github::status_checks_by_sha;

use serde_json::Value;

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct StatusCheckData {
    name: String,
    status: String,
    conclusion: String
}

impl StatusCheckData {
  pub fn new(name: &str, status: &str, conclusion: &str) -> StatusCheckData {
      StatusCheckData {
          name: name.to_string(),
          status: status.to_string(),
          conclusion: conclusion.to_string()
      }
  }
  pub fn get_name(&self) -> &str {
    return &self.name;
  }
  pub fn get_status(&self) -> &str {
    return &self.status;
  }
  pub fn get_conclusion(&self) -> &str {
    return &self.conclusion;
  }
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PrCommitData {
    title: String,
    merged_at: String,
    merge_commit_sha: String,
    statuses: Vec<StatusCheckData>
}

impl PrCommitData {
  pub fn new(title: &str, merged_at: &str, merge_commit_sha: &str, status_array: Vec<StatusCheckData>) -> PrCommitData {
      PrCommitData {
          title: title.to_string(),
          merged_at: merged_at.to_string(),
          merge_commit_sha: merge_commit_sha.to_string(),
          statuses: status_array
      }
  }
  pub fn get_title(&self) -> &str {
    return &self.title;
  }
  pub fn get_merged_at(&self) -> &str {
    return &self.merged_at;

  }
  pub fn get_merge_commit_sha(&self) -> &str {
      return &self.merge_commit_sha;
  }
  pub fn get_statuses(&self) -> &Vec<StatusCheckData> {
    return &self.statuses;

  }
}

pub fn repo_api_base(repo_url: &str) -> Option<String> {
  let trimmed = repo_url.trim_end_matches('/');
  if trimmed.starts_with("https://api.github.com/repos/") {
    return Some(format!("{}/", trimmed));
  }
  if let Some(path) = trimmed.strip_prefix("https://github.com/") {
    let repo_path = path.trim_end_matches(".git");
    return Some(format!("https://api.github.com/repos/{}/", repo_path));
  }
  if let Some(path) = trimmed.strip_prefix("http://github.com/") {
    let repo_path = path.trim_end_matches(".git");
    return Some(format!("https://api.github.com/repos/{}/", repo_path));
  }
  if let Some(path) = trimmed.strip_prefix("git@github.com:") {
    let repo_path = path.trim_end_matches(".git");
    return Some(format!("https://api.github.com/repos/{}/", repo_path));
  }
  None
}

pub async fn poll_merge_commits(repo: &str, branch: &str, last_sha: Option<&str>) -> Vec<PrCommitData> {

  let merge_commits_json: Vec<Value> = match merge_commits::get_merge_commits(repo, branch).await {
    Ok(value) => value,
    Err(_) => return Vec::new(),
  };

  let mut commits_array: Vec<PrCommitData> = Vec::new();

  for i in &merge_commits_json {
    let merge_sha = match i["merge_commit_sha"].as_str() {
      Some(value) => value,
      None => continue,
    };
    if let Some(saved_sha) = last_sha {
      if merge_sha == saved_sha {
        break;
      }
    }
    let status_checks_json: Value = match status_checks_by_sha::get_status_checks_by_sha(repo, merge_sha).await {
      Ok(value) => value,
      Err(_) => continue,
    };
    let mut status_array: Vec<StatusCheckData> = Vec::new();
    if let Some(statuses) = status_checks_json["check_runs"].as_array() {
      for status in statuses {
        let status_data = StatusCheckData::new(&status["name"].to_string(), &status["status"].to_string(), &status["conclusion"].to_string());
        status_array.push(status_data);
      }
    }
    let data = PrCommitData::new(&i["title"].to_string(), &i["merged_at"].to_string(), merge_sha, status_array);

    commits_array.push(data);
  }

  println!("\n\nNautilus is displaying undeployed commits for repo {}:\n", repo);
  for i in &commits_array {
    println!("----------\nTitle: {}\nMerged: {}\nMerge Commit Sha: {}\n", i.get_title(), i.get_merged_at(), i.get_merge_commit_sha());
    let statuses = i.get_statuses();
    println!("Reporting Status Checks...\n");
    if statuses.is_empty() {
      println!("No Status Checks associated with this Commit\n");
    }
    else {
      for j in statuses {
        println!{"Check: {}, Status: {}, Result: {}\n", j.get_name(), j.get_status(), j.get_conclusion()}
      }
    }
  }
  commits_array
}

