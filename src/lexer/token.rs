use std::ops;

use super::span::SpanRange;

#[derive(Clone)]
pub struct SpannedToken {
    pub span: SpanRange,
    pub token: Token,
}

#[derive(Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Keyword(TokenKeyword),
    Literal(TokenLiteral),
    Punctuation(TokenPunctuation),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKeyword {
    // Def,
    Elif,
    Else,
    // For,
    Global,
    If,
    // Loop,
    // While,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenLiteral {
    Nil,
    Bool(bool),
    Number(i64),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenPunctuation {
    /// +
    Add,
    /// !
    Bang,
    /// !=
    BangEqual,
    /// :
    Colon,
    /// ,
    Comma,
    /// =
    Equal,
    /// ==
    EqualEqual,
    /// <Tab>
    Indentation,
    /// -
    Minus,
    /// \n
    Newline,
    /// (
    ParenOpen,
    /// )
    ParenClose,
    /// #
    Pound,
    /// /
    Slash,
    /// *
    Star,
}

impl Token {
    pub fn into_ident(self) -> Option<String> {
        if let Self::Ident(ident) = self {
            Some(ident)
        } else {
            None
        }
    }
}

impl SpannedToken {
    pub fn new(span: SpanRange, token: Token) -> Self {
        Self { span, token }
    }

    pub fn parts(self) -> (SpanRange, Token) {
        (self.span, self.token)
    }
}

impl ops::Deref for SpannedToken {
    type Target = Token;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

impl PartialEq for SpannedToken {
    fn eq(&self, other: &SpannedToken) -> bool {
        self.token == other.token
    }
}

impl PartialEq<Token> for SpannedToken {
    fn eq(&self, other: &Token) -> bool {
        &self.token == other
    }
}
