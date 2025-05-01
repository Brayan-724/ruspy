use std::fs;
use std::path::PathBuf;

use ruspy::bin::required_first_arg;
use ruspy::lexer::Lexer;

fn main() {
    let file_path: PathBuf = required_first_arg();
    let file_content = fs::read_to_string(&file_path)
        .unwrap_or_else(|err| panic!("Cannot read {file_path:?}: {err}"));

    _ = Lexer::from_str(&mut file_content.as_str())
        .inspect(|t| Lexer::pretty_print(t))
        .inspect_err(|err| eprintln!("{err:#?}"));
}
