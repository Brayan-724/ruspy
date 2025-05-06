use winnow::Parser;
use winnow::stream::AsChar;
use winnow::token::take_while;

use super::source::{LexerResult, SourceLexer};
use super::token::TokenLiteral;

pub fn eat_spaces<'i>(input: &mut SourceLexer<'i>) -> LexerResult<'i> {
    take_while(0.., AsChar::is_space).parse_next(input)?;
    Ok(())
}

#[macro_export]
macro_rules! ident {
    (@raw $i:ident) => {
        String::from(stringify!($i))
    };
    (@raw $i:literal) => {
        String::from($i)
    };
    (@ast $i:ident) => {
        $crate::ast::node::AstExpr::Ident(String::from(stringify!($i)))
    };
    (@ast $i:literal) => {
        $crate::ast::node::AstExpr::Ident(String::from($i))
    };
    ($i:ident) => {
        $crate::lexer::token::Token::Ident(String::from(stringify!($i)))
    };
    ($i:literal) => {
        $crate::lexer::token::Token::Token::Ident(String::from($i))
    };
}

#[macro_export]
macro_rules! literal {
    (@raw $i:expr) => {
        Into::<$crate::lexer::token::TokenLiteral>::into($i)
    };
    (@ast $i:expr) => {
        $crate::ast::node::AstExpr::Literal(Into::<$crate::lexer::token::TokenLiteral>::into($i))
    };
    ($i:expr) => {
        $crate::lexer::token::Token::Literal(Into::<$crate::lexer::token::TokenLiteral>::into($i))
    };
}

#[macro_export]
macro_rules! T {
    (@raw $i:ident) => {
        $crate::lexer::token::TokenPunctuation::$i
    };
    ($i:ident) => {
        $crate::lexer::token::Token::Punctuation($crate::lexer::token::TokenPunctuation::$i)
    };
}

#[macro_export]
macro_rules! kw {
    (@raw $i:ident) => {
        $crate::lexer::token::TokenKeyword::$i
    };
    ($i:ident) => {
        $crate::lexer::token::Token::Keyword($crate::lexer::token::TokenKeyword::$i)
    };
}

pub use T;
pub use ident;
pub use kw;
pub use literal;

impl From<i64> for TokenLiteral {
    fn from(value: i64) -> Self {
        Self::Number(value)
    }
}

impl From<bool> for TokenLiteral {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for TokenLiteral {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for TokenLiteral {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}
