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
    let client = reqwest::blocking::Client::new();
    let mut last_seen: Option<String> = None;
    let token = std::env::var("ZEVA_GITHUB_TOKEN").ok();

    loop {
        let mut request = client
            .get("https://api.github.com/repos/bcgov/zeva/commits/test-naultilus")
            .header("User-Agent", "nautilus-hackathon-2026");

        if let Some(ref token) = token {
            request = request.bearer_auth(token);
        }

        match request.send() {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Connected to GitHub API for bcgov/zeva.");
                }
                match response.json::<Commit>() {
                    Ok(commit) => {
                        if last_seen.as_deref() != Some(commit.sha.as_str()) {
                            println!("New commit on test-naultilus: {}", commit.sha);
                            last_seen = Some(commit.sha);
                        }
                    }
                    Err(err) => println!("Failed to parse GitHub response: {}", err),
                }
            }
            Err(err) => println!("Failed to reach GitHub API: {}", err),
        }

        thread::sleep(Duration::from_secs(30));
    }
}
