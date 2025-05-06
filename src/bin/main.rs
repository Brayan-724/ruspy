use std::fs;
use std::path::PathBuf;

use ruspy::bin::required_first_arg;

fn main() {
    let file_path: PathBuf = required_first_arg();
    let file_content = fs::read_to_string(&file_path)
        .unwrap_or_else(|err| panic!("Cannot read {file_path:?}: {err}"));

    println!("{file_content}");
}
