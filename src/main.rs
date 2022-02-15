// use std::process::{exit, Command};
use open_repo::*;

fn main() {
    check_git();
    let remote_url = get_remote_url();
    println!("remote url -> {}", remote_url);
    println!("final url -> {}", generate_url(&remote_url));
}
