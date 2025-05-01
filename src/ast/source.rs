use core::fmt;
use std::collections::VecDeque;
use std::ops;

use crate::common::error::{raise_at, raise_range};
use crate::lexer::span::SpanRange;
use crate::lexer::token::{SpannedToken, Token};

#[derive(Clone, Debug)]
pub struct SourceAst<'i> {
    pub base: &'i str,
    pub last_offset: usize,
    pub tokens: VecDeque<SpannedToken>,
}

pub struct PeekedToken<'i, 's> {
    pub source: &'s mut SourceAst<'i>,
    pub token: SpannedToken,
    pub last_offset: usize,
}

impl<'i> SourceAst<'i> {
    pub fn new(base: &'i str, tokens: VecDeque<SpannedToken>) -> Self {
        Self {
            base,
            tokens,
            last_offset: 0,
        }
    }

    pub fn with(&self, tokens: VecDeque<SpannedToken>) -> Self {
        Self {
            tokens,
            base: self.base,
            last_offset: self.last_offset,
        }
    }

    pub fn peek<'a>(&'a mut self) -> Option<PeekedToken<'i, 'a>> {
        self.tokens.pop_front().map(|token| {
            let last_offset = self.last_offset;
            self.last_offset = token.span.from.offset;
            PeekedToken {
                token,
                last_offset,
                source: self,
            }
        })
    }

    pub fn peek_expect<'a>(&'a mut self) -> PeekedToken<'i, 'a> {
        // The implementation cannot be done with `peek` call
        // because of borrow checker :|
        let Some(token) = self.tokens.pop_front() else {
            self.error_in_place("Unexpected EOF");
        };

        let last_offset = self.last_offset;
        self.last_offset = token.span.from.offset;
        PeekedToken {
            token,
            last_offset,
            source: self,
        }
    }

    pub fn expect(&mut self) -> SpannedToken {
        self.tokens
            .pop_front()
            .inspect(|t| self.last_offset = t.span.from.offset)
            .unwrap_or_else(|| self.error_in_place("Unexpected EOF"))
    }

    pub fn expect_msg(&mut self, msg: impl fmt::Display) -> SpannedToken {
        self.tokens
            .pop_front()
            .inspect(|t| self.last_offset = t.span.from.offset)
            .unwrap_or_else(|| self.error_in_place(format!("Unexpected EOF. {msg}")))
    }

    pub fn expect_match<T>(
        &mut self,
        msg: impl fmt::Display,
        predicate: impl Fn(SpannedToken) -> Option<T>,
    ) -> T {
        let first = self.expect_msg(format!("Expected {msg}"));

        let span = first.span;
        let err = format!("Unexpected token: {first:?}. Expected: {msg}");

        if let Some(t) = predicate(first) {
            self.last_offset = span.from.offset;
            t
        } else {
            self.error_at(span, err);
        }
    }

    pub fn expect_token(&mut self, token: Token) -> SpannedToken {
        self.expect_match(format!("{token:#?}"), |t| (t == token).then_some(t))
    }

    pub fn error_in_place(&self, msg: impl fmt::Display) -> ! {
        raise_at(self.base, self.last_offset, msg)
    }

    pub fn error_at(&self, span: SpanRange, msg: impl fmt::Display) -> ! {
        raise_range(self.base, span, msg)
    }
}

impl<'i, 's> PeekedToken<'i, 's> {
    pub fn accept(self) -> SpannedToken {
        self.token
    }

    pub fn recover(self) {
        self.source.last_offset = self.last_offset;
        self.source.tokens.push_front(self.token);
    }
}

impl<'i, 's> ops::Deref for PeekedToken<'i, 's> {
    type Target = SpannedToken;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}
