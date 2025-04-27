use std::fs;
use std::path::PathBuf;

use ruspy::bin::required_first_arg;
use ruspy::lexer::Lexer;

fn main() {
    let file_path: PathBuf = required_first_arg();
    let file_content = fs::read_to_string(&file_path)
        .unwrap_or_else(|err| panic!("Cannot read {file_path:?}: {err}"));

    let tokens = Lexer::from_str(&mut file_content.as_str());

    match tokens {
        Ok(tokens) => {
            println!("LIN:COL LIN:COL KIND        RENDER");
            tokens.into_iter().for_each(|token| println!("{token:#?}"))
        }
        Err(err) => eprintln!("{err:#?}"),
    }
}
