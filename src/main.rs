use open;
use open_repo::*;
use std::env;

fn main() -> Result<(), &'static str> {
    check_git();
    let remote_url = get_remote_url();
    let args: Vec<String> = env::args().collect();
    let mut url = generate_url(&remote_url);
    if url.contains("github.com") {
        let subpath = generate_sub_path(&args)?;
        url.push_str(subpath.as_str());
    }
    println!("Opening url: {}", &url);
    match open::that(&url) {
        Err(_) => Err("Failed to open repo."),
        _ => Ok(()),
    }
}
