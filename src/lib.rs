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
        .stdout(std::process::Stdio::piped()) // Redirect stderr to stdout
        .stderr(std::process::Stdio::piped())
        .output();
    match result {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.trim().to_string()
            } else {
                // Custom error message without printing detailed error from git command
                println!("Error: Not in a git repository or failed to get remote URL.");
                exit(output.status.code().unwrap_or(1));
            }
        },
        Err(e) => {
            // Handle errors when executing git command
            eprintln!("Failed to run git command: {}", e);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generate_correct_url() {
        let url = "git@github.com:Meowu/open-repo.git";
        let url1 = "https://github.com/Meowu/open-repo.git";
        let result = String::from("https://github.com/Meowu/open-repo");
        assert_eq!(generate_url(url), result);
        assert_eq!(generate_url(url1), result);
    }
}