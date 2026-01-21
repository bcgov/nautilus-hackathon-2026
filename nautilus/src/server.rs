use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;

use serde::Deserialize;

pub fn run() {
    thread::spawn(poll_zeva_branch);
    println!("Starting server on 0.0.0.0:8080");
    let listener = TcpListener::bind("0.0.0.0:8080").expect("bind 0.0.0.0:8080");

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let mut buffer = [0_u8; 512];
            let _ = stream.read(&mut buffer);
            let request_line = std::str::from_utf8(&buffer)
                .ok()
                .and_then(|text| text.lines().next())
                .unwrap_or("");
            let path = request_line
                .split_whitespace()
                .nth(1)
                .unwrap_or("/");
            let body = if path == "/healthz" { "ok" } else { "Hello world" };
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes());
        }
    }
}

#[derive(Deserialize)]
struct Commit {
    sha: String,
}

const REPO_URL: &str = "https://github.com/bcgov/nautilus-test-repo.git";
const REPO_BRANCH: &str = "devops/test-commit-read";
const REPO_DIR: &str = "/app/nautilus-test-repo";

fn poll_zeva_branch() {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(0)
        .build()
        .expect("build reqwest client");
    let mut last_seen: Option<String> = None;
    let mut logged_connected = false;
    let token = std::env::var("NAUTILUSTEST_GITHUB_TOKEN").ok();
    if token.is_none() {
        println!("NAUTILUSTEST_GITHUB_TOKEN is not set; git operations may fail.");
    }

    loop {
        let mut last_error: Option<String> = None;
        let mut attempts_left = 3;
        let mut backoff_secs = 1;

        while attempts_left > 0 {
            let mut request = client
                .get("https://api.github.com/repos/bcgov/nautilus-test-repo/commits/devops/test-commit-read")
                .header("User-Agent", "nautilus-hackathon-2026");

            if let Some(ref token) = token {
                request = request.bearer_auth(token);
            }

            match request.send() {
                Ok(response) => {
                    let status = response.status();
                    let body = response.text().unwrap_or_default();
                    if status.is_success() {
                        if !logged_connected {
                            println!("Connected to GitHub API for bcgov/nautilus-test-repo.");
                            logged_connected = true;
                        }
                        match serde_json::from_str::<Commit>(&body) {
                            Ok(commit) => {
                                if last_seen.as_deref() != Some(commit.sha.as_str()) {
                                    println!(
                                        "New commit on devops/test-commit-read: {}",
                                        commit.sha
                                    );
                                    last_seen = Some(commit.sha);
                                    if let Err(err) = sync_repo_and_run(token.as_deref()) {
                                        println!("Failed to sync repo or run script: {}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                last_error = Some(format!(
                                    "Failed to parse GitHub response: {} | body={}",
                                    err, body
                                ));
                            }
                        }
                    } else {
                        last_error = Some(format!(
                            "GitHub API returned {}: {}",
                            status.as_u16(),
                            body
                        ));
                    }
                    break;
                }
                Err(err) => {
                    last_error = Some(format!("Failed to reach GitHub API: {}", err));
                }
            }

            attempts_left -= 1;
            if attempts_left > 0 {
                thread::sleep(Duration::from_secs(backoff_secs));
                backoff_secs = (backoff_secs * 2).min(8);
            }
        }

        if let Some(err) = last_error {
            println!("{}", err);
        }

        thread::sleep(Duration::from_secs(30));
    }
}

fn sync_repo_and_run(token: Option<&str>) -> Result<(), String> {
    if !Path::new(REPO_DIR).exists() {
        let repo_url = build_repo_url(token);
        let status = Command::new("git")
            .arg("clone")
            .arg("--branch")
            .arg(REPO_BRANCH)
            .arg("--single-branch")
            .arg(repo_url)
            .arg(REPO_DIR)
            .env("GIT_TERMINAL_PROMPT", "0")
            .status()
            .map_err(|err| format!("git clone failed: {}", err))?;
        if !status.success() {
            return Err(format!("git clone exited with {}", status));
        }
    }

    let status = Command::new("git")
        .arg("-C")
        .arg(REPO_DIR)
        .arg("fetch")
        .arg("origin")
        .arg(REPO_BRANCH)
        .env("GIT_TERMINAL_PROMPT", "0")
        .status()
        .map_err(|err| format!("git fetch failed: {}", err))?;
    if !status.success() {
        return Err(format!("git fetch exited with {}", status));
    }

    let status = Command::new("git")
        .arg("-C")
        .arg(REPO_DIR)
        .arg("checkout")
        .arg(REPO_BRANCH)
        .env("GIT_TERMINAL_PROMPT", "0")
        .status()
        .map_err(|err| format!("git checkout failed: {}", err))?;
    if !status.success() {
        return Err(format!("git checkout exited with {}", status));
    }

    let status = Command::new("git")
        .arg("-C")
        .arg(REPO_DIR)
        .arg("reset")
        .arg("--hard")
        .arg(format!("origin/{}", REPO_BRANCH))
        .env("GIT_TERMINAL_PROMPT", "0")
        .status()
        .map_err(|err| format!("git reset failed: {}", err))?;
    if !status.success() {
        return Err(format!("git reset exited with {}", status));
    }

    let status = Command::new("sh")
        .arg("./nautilus.sh")
        .current_dir(REPO_DIR)
        .status()
        .map_err(|err| format!("nautilus.sh failed: {}", err))?;
    if !status.success() {
        return Err(format!("nautilus.sh exited with {}", status));
    }

    Ok(())
}

fn build_repo_url(token: Option<&str>) -> String {
    match token {
        Some(token) => format!("https://x-access-token:{}@github.com/bcgov/nautilus-test-repo.git", token),
        None => REPO_URL.to_string(),
    }
}
