use std::process::Stdio;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

pub async fn run_shell_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = Command::new("/bin/sh");

    command.args(["-c", file_path]);
    command.stdout(Stdio::piped());

    let mut child = command.spawn().expect("Failed to spawn bash subprocess");
    let stdout = child.stdout.take().expect("No stdout available");

    let mut reader = BufReader::new(stdout).lines();

    tokio::spawn(async move {
        let status = child.wait().await.expect("Error in bash child process");
        println!("Bash subprocess exited with status {}", status);
    });

    while let Some(line) = reader.next_line().await? {
        println!("Received: {}", line);
    }

    Ok(())
}
