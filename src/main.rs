use open;
use open_repo::*;

fn main() {
    check_git();
    let remote_url = get_remote_url();
    let url = generate_url(&remote_url);
    open::that(&url);
}
