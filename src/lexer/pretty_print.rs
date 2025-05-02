use core::fmt;
use std::collections::VecDeque;
use std::fmt::Write;

use crate::pretty_print::*;

use super::Lexer;
use super::span::{Span, SpanRange};
use super::token::{SpannedToken, Token, TokenKeyword, TokenLiteral, TokenPunctuation};

impl Lexer {
    pub fn pretty_print(tokens: &VecDeque<SpannedToken>) {
        println!("LIN:COL LIN:COL KIND        RENDER");
        tokens.iter().for_each(|token| println!("{token}"));
    }
}

impl fmt::Debug for SpannedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(&self.token, f)
    }
}

impl fmt::Display for SpannedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({:#}) {}", self.span, self.token))
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!(
                "{LINE}{:>2}{R}:{COLUMN}{:>2}{R}",
                self.line, self.col
            ))
        } else {
            f.write_fmt(format_args!("{} [{}:{}]", self.offset, self.line, self.col))
        }
    }
}

impl fmt::Display for SpanRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("{:#} ..{:#}", self.from, self.to))
        } else {
            f.write_fmt(format_args!("({:})..({:})", self.from, self.to))
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(ident) => f.write_fmt(format_args!("Ident({ident})"))?,
            Token::Keyword(kw) => f.write_fmt(format_args!("{kw:?}"))?,
            Token::Literal(lit) => f.write_fmt(format_args!("{lit:?}"))?,
            Token::Punctuation(punctuation) => f.write_fmt(format_args!("{punctuation:?}"))?,
        }

        Ok(())
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(ident) => f.write_fmt(format_args!("{VARIABLE}Ident       {ident}{R}"))?,
            Token::Keyword(kw) => f.write_fmt(format_args!("{KEYWORD}Keyword     {kw:}"))?,
            Token::Literal(lit) => f.write_fmt(format_args!("{LITERAL}Literal     {lit:}"))?,
            Token::Punctuation(punctuation) => {
                f.write_fmt(format_args!("Punctuation {punctuation}"))?
            }
        }

        Ok(())
    }
}

impl fmt::Display for TokenKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(KEYWORD)?;
        match self {
            TokenKeyword::Elif => f.write_str("elif")?,
            TokenKeyword::Else => f.write_str("else")?,
            TokenKeyword::Global => f.write_str("global")?,
            TokenKeyword::If => f.write_str("if")?,
        }
        f.write_str(R)
    }
}

impl fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenLiteral::Nil => f.write_fmt(format_args!("{LITERAL}nil"))?,
            TokenLiteral::Bool(true) => f.write_fmt(format_args!("{LITERAL}True"))?,
            TokenLiteral::Bool(false) => f.write_fmt(format_args!("{LITERAL}False"))?,
            TokenLiteral::Number(n) => f.write_fmt(format_args!("{LITERAL}{n}"))?,
            TokenLiteral::String(s) => f.write_fmt(format_args!("{LITERAL}{s:?}"))?,
        }
        f.write_str(R)
    }
}

impl fmt::Display for TokenPunctuation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenPunctuation::Add => f.write_char('+'),
            TokenPunctuation::Bang => f.write_char('!'),
            TokenPunctuation::BangEqual => f.write_str("!="),
            TokenPunctuation::Colon => f.write_char(':'),
            TokenPunctuation::Comma => f.write_char(','),
            TokenPunctuation::Equal => f.write_char('='),
            TokenPunctuation::EqualEqual => f.write_str("=="),
            TokenPunctuation::Indentation => f.write_char(' '),
            TokenPunctuation::Minus => f.write_char('-'),
            TokenPunctuation::Newline => f.write_str("\\n"),
            TokenPunctuation::ParenOpen => f.write_char('('),
            TokenPunctuation::ParenClose => f.write_char(')'),
            TokenPunctuation::Pound => f.write_char('#'),
            TokenPunctuation::Slash => f.write_char('/'),
            TokenPunctuation::Star => f.write_char('*'),
        }
    }
}
