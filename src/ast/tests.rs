use crate::ast::node::{AstScope, AstStatement};
use crate::ast::utils::{bin_op, scope};
use crate::lexer::Lexer;
use crate::lexer::utils::{ident, literal};

fn create_scope(content: &str) -> AstScope {
    AstScope::from_tokens(content, Lexer::from_str(content).unwrap())
}

#[test]
fn simple() {
    let res = create_scope("a = 1");
    assert_eq!(
        res,
        scope![AstStatement::VariableDeclaration(
            ident!(@raw a),
            literal!(@ast 1).into()
        )]
    )
}

#[test]
fn expression() {
    let res = create_scope("a + b * c - d / e");
    assert_eq!(
        res,
        scope![AstStatement::Expresion(
            bin_op!(
                ident!(@ast a),
                Add,
                bin_op!(
                    bin_op!(ident!(@ast b), Mul, ident!(@ast c)),
                    Sub,
                    bin_op!(ident!(@ast d), Div, ident!(@ast e))
                )
            )
            .into()
        )]
    )
}

#[test]
fn conditional() {
    let res = create_scope("if True:\n a = 1");
    assert_eq!(
        res,
        scope![AstStatement::Conditional {
            test: literal!(@ast true).into(),
            body: scope!(AstStatement::VariableDeclaration(
                ident!(@raw a),
                literal!(@ast 1).into()
            )),
            otherwise: None
        }]
    )
}

#[test]
fn conditional_inline() {
    let res = create_scope("if True: a = 1");
    assert_eq!(
        res,
        scope![AstStatement::Conditional {
            test: literal!(@ast true).into(),
            body: scope!(AstStatement::VariableDeclaration(
                ident!(@raw a),
                literal!(@ast 1).into()
            )),
            otherwise: None
        }]
    )
}

#[test]
fn conditional_else() {
    let res = create_scope("if True:\n a = 1\nelse:\n a = 2");
    assert_eq!(
        res,
        scope![AstStatement::Conditional {
            test: literal!(@ast true).into(),
            body: scope!(AstStatement::VariableDeclaration(
                ident!(@raw a),
                literal!(@ast 1).into()
            )),
            otherwise: Some(scope!(AstStatement::VariableDeclaration(
                ident!(@raw a),
                literal!(@ast 2).into()
            )))
        }]
    )
}
