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

    println!(" -- LEXER --");

    let tokens = Lexer::from_str(&mut file_content.as_str()).expect("Cannot parse");

    println!("LIN:COL LIN:COL KIND        RENDER");
    tokens.iter().for_each(|token| println!("{token:#?}"));

    println!(" -- AST --");

    let tree = AstScope::from_tokens(&file_content, tokens);

    println!("{tree:#?}");

    println!(" -- AST (pretty) --");
    println!("{tree}");

    println!(" -- OUTPUT --");

    let scope = Scope::new();
    scope.run(tree);

    println!("{scope:#?}");
}
