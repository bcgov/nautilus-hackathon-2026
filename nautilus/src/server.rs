use std::io::{Read, Write};
use std::net::TcpListener;
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

fn poll_zeva_branch() {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .pool_max_idle_per_host(0)
        .build()
        .expect("build reqwest client");
    let mut last_seen: Option<String> = None;
    let mut logged_connected = false;
    let token = std::env::var("NAUTILUSTEST_GITHUB_TOKEN").ok();

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
