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