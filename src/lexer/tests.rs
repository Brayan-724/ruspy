use crate::lexer::Lexer;
use crate::lexer::utils::{T, ident, kw, literal};

#[test]
fn simple() {
    let res = Lexer::from_str("a = 1").unwrap();
    assert_eq!(Vec::from(res), &[ident!(a), T!(Equal), literal!(1)])
}

#[test]
fn expression() {
    let res = Lexer::from_str("a + b * c - d / e").unwrap();
    assert_eq!(
        Vec::from(res),
        &[
            ident!(a),
            T!(Add),
            ident!(b),
            T!(Star),
            ident!(c),
            T!(Minus),
            ident!(d),
            T!(Slash),
            ident!(e)
        ]
    )
}

#[test]
fn keywords() {
    let res = Lexer::from_str("global").unwrap();
    assert_eq!(Vec::from(res), &[kw!(Global)])
}

#[test]
fn conditional() {
    let res = Lexer::from_str(
        "if True:\
       \n a = 1",
    )
    .unwrap();
    assert_eq!(
        Vec::from(res),
        &[
            kw!(If),
            literal!(true),
            T!(Colon),
            T!(Newline),
            T!(Indentation),
            ident!(a),
            T!(Equal),
            literal!(1)
        ]
    )
}
