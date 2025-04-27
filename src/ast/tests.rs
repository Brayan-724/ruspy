use std::collections::VecDeque;

use crate::ast::node::{AstScope, AstStatement};
use crate::ast::utils::{bin_op, scope};
use crate::lexer::span::Span;
use crate::lexer::token::{SpannedToken, Token};
use crate::lexer::utils::{T, ident, kw, literal};

fn zero_spanned(input: VecDeque<Token>) -> VecDeque<SpannedToken> {
    input
        .into_iter()
        .map(|token| SpannedToken::new(Span::zeroed().range_to(Span::zeroed()), token))
        .collect()
}

#[test]
fn simple() {
    let input = [ident!(a), T!(Equal), literal!(1)].to_vec();
    let res = AstScope::from_tokens(zero_spanned(input.into()));
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
    let input = [
        ident!(a),
        T!(Add),
        ident!(b),
        T!(Star),
        ident!(c),
        T!(Minus),
        ident!(d),
        T!(Slash),
        ident!(e),
    ]
    .to_vec();
    let res = AstScope::from_tokens(zero_spanned(input.into()));
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
    let input = [
        kw!(If),
        literal!(true),
        T!(Colon),
        T!(Newline),
        T!(Indentation),
        ident!(a),
        T!(Equal),
        literal!(1),
    ]
    .to_vec();
    let res = AstScope::from_tokens(zero_spanned(input.into()));
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
    let input = [
        kw!(If),
        literal!(true),
        T!(Colon),
        ident!(a),
        T!(Equal),
        literal!(1),
    ]
    .to_vec();
    let res = AstScope::from_tokens(zero_spanned(input.into()));
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
    #[rustfmt::skip]
    let input = [
        kw!(If), literal!(true), T!(Colon), T!(Newline),
        T!(Indentation), ident!(a), T!(Equal), literal!(1), T!(Newline),
        kw!(Else), T!(Colon), T!(Newline),
        T!(Indentation), ident!(a), T!(Equal), literal!(2),
    ]
    .to_vec();
    let res = AstScope::from_tokens(zero_spanned(input.into()));
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
