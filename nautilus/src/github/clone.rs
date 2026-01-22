
use std::{process::Command};

use crate::{environment::getenv};

fn create_directory(sha: &str) -> Result<String, std::io::Error> {


    let base_dir = getenv("WORKSPACE", "./workspace");
    let workspace_dir = base_dir + "/" + sha;

    std::fs::remove_dir(workspace_dir.as_str()).ok();
    std::fs::create_dir_all(workspace_dir.as_str())?;

    Ok(workspace_dir)
}

pub fn by_sha(repo_url: &str, sha: &str) -> Result<String, String> {
    let workspace_path = create_directory(sha).unwrap();

    println!("Cloning into {workspace_path}");

    Command::new("git").args(&[
        "clone",
        "--depth",
        "1",
        "--revision",
        sha,
        repo_url,
        workspace_path.as_str()
    ]).status().ok();

    Ok(workspace_path)
}
