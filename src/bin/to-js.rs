use std::fs;
use std::path::PathBuf;

use ruspy::ast::node::AstScope;
use ruspy::bin::required_first_arg;
use ruspy::compiler::Compiler;
use ruspy::compiler::js::JsCompiler;
use ruspy::lexer::Lexer;

fn main() {
    let file_path: PathBuf = required_first_arg();
    let file_content = fs::read_to_string(&file_path)
        .unwrap_or_else(|err| panic!("Cannot read {file_path:?}: {err}"));

    let tokens = Lexer::from_str(&file_content).expect("Cannot parse");

    let tree = AstScope::from_tokens(&file_content, tokens);

    let mut output = String::new();

    JsCompiler.visit_scope(&mut output, tree).unwrap();

    println!("{output}");
}
