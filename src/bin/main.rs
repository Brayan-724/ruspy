use std::fs;
use std::path::PathBuf;

use ruspy::ast::node::AstScope;
use ruspy::bin::required_first_arg;
use ruspy::lexer::Lexer;
use ruspy::runtime::Scope;

fn main() {
    let file_path: PathBuf = required_first_arg();
    let file_content = fs::read_to_string(&file_path)
        .unwrap_or_else(|err| panic!("Cannot read {file_path:?}: {err}"));

    let tokens = Lexer::from_str(&mut file_content.as_str()).expect("Cannot parse");

    let tree = AstScope::from_tokens(tokens);

    let scope = Scope::new();
    scope.run(tree);

    println!("{scope:#?}")
}
