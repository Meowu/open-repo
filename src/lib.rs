use open;
use std::process::{exit, Command};

pub fn check_git() {
    let command = "git";
    let result = Command::new(command).arg("--version").output();
    if let Err(_) = result {
        println!("{} not found.", command);
        exit(1);
    }
}

fn get_arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|arg| arg == flag)
        .and_then(|index| args.get(index + 1).cloned())
}

pub fn open_profile(args: &[String]) -> Result<(), &'static str> {
    if let Some(name) = get_arg_value(&args, "-p").or_else(|| get_arg_value(&args, "--profile")) {
        let profile_url = format!("https://github.com/{}", name);
        match open::that(profile_url) {
            Err(_) => Err("Failed to open profile."),
            _ => Ok(()),
        }
    } else {
        Err("Not profile.")
    }
}

pub fn generate_sub_path(args: &[String]) -> Result<String, &'static str> {
    let open_issue = args.iter().any(|arg| arg == "--issue");
    let open_pull = args.contains(&String::from("--pull"));

    if open_issue && open_pull {
        return Err("Cannot use both --issue and --pull at the same time");
    }
    if open_issue {
        if let Some(issue) = get_arg_value(&args, "--issue") {
            return Ok(format!("/issues/{}", issue));
        } else {
            return Ok("/issues".to_string());
        }
    }
    if open_pull {
        if let Some(pull) = get_arg_value(&args, "--pull") {
            return Ok(format!("/pull/{}", pull));
        } else {
            return Ok("/pulls".to_string());
        }
    }
    Ok(String::new())
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
        }
        Err(e) => {
            // Handle errors when executing git command
            eprintln!("Failed to run git command: {}", e);
            exit(1);
        }
    }
}

pub fn generate_url(remote_url: &str) -> String {
    let url = remote_url.trim_end_matches(".git");
    
    if url.starts_with("https://") || url.starts_with("http://") {
        url.to_string()
    } else if url.starts_with("git@") {
        // 处理 SSH 格式: git@github.com:user/repo
        url.replacen(":", "/", 1)
           .replace("git@", "https://")
    } else {
        url.to_string()
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
