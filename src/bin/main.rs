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

    let tokens = Lexer::from_str(&file_content)
        .inspect(Lexer::pretty_print)
        .unwrap_or_else(|err| panic!("{err:#?}"));

    println!(" -- AST --");

    let tree = AstScope::from_tokens(&file_content, tokens);

    println!("{tree:#?}");

    println!(" -- AST (pretty) --");
    println!("{tree}");

    let scope = Scope::new();

    scope.run(tree);

    println!("{scope:#?}");
}
