use std::process::{Command, exit};

pub fn check_git() {
    let command = "git";
    let result = Command::new(command)
        .arg("--version")
        .output();
    if let Err(_) = result {
        println!("{} not found.", command);
        exit(1);
    }
}

pub fn get_remote_url() -> String {
    let result = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output();
    match result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.trim().to_string()
        },
        Err(_) => {
            println!("not in a repository.");
            exit(1);
        }
    }
}

pub fn generate_url(remot_url: &str) -> String {
    let url = remot_url.replace(".git", "");
    if url.starts_with("https") {
        url
    } else {
        url.replace(":", "/").replace("git@", "https://")
    }
}