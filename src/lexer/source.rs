use core::fmt;
use std::backtrace::Backtrace;
use std::ops;

use winnow::error::{AddContext, ParserError};
use winnow::stream::{AsChar, FindSlice, Offset, Stream, StreamIsPartial};

use crate::common::error::{ctx_line, raise_span};

use super::span::{Span, SpanRange};

#[derive(Debug, Clone)]
pub struct SourceLexer<T> {
    base: T,
    data: T,
    last_col: usize,
    pub line: usize,
    pub col: usize,
    pub offset: usize,
}

#[derive(Debug)]
pub struct LexerError {
    span: Span,
    labels: Vec<(SpanRange, String, String)>,
    backtrace: Backtrace,
}

impl<'i> SourceLexer<&'i str> {
    pub fn new(data: &'i str) -> Self {
        Self {
            data,
            base: data,
            last_col: 0,
            line: 0,
            col: 0,
            offset: 0,
        }
    }

    /// Get span of current token.
    /// Or previous if is newline
    pub fn span(&self) -> Span {
        if &self.base[self.offset..self.base.len().min(self.offset + 1)] == "\n" {
            let (line, col) = if self.col == 0 {
                (self.line.saturating_sub(1), self.last_col)
            } else {
                (self.line, unsafe { self.col.unchecked_sub(1) })
            };

            Span {
                line,
                col,
                offset: self.offset.saturating_sub(1),
            }
        } else {
            Span {
                line: self.line,
                col: self.col,
                offset: self.offset,
            }
        }
    }

    pub fn error(&self, msg: impl fmt::Display) -> ! {
        let span = self.span();
        raise_span(self.base, span, msg)
    }
}

impl<T> ops::Deref for SourceLexer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<U, T: AsRef<U>> AsRef<U> for SourceLexer<T> {
    fn as_ref(&self) -> &U {
        self.data.as_ref()
    }
}

impl<T> Offset<SourceLexer<T>> for SourceLexer<T> {
    fn offset_from(&self, start: &SourceLexer<T>) -> usize {
        self.offset - start.offset
    }
}

impl<U: Clone, T: FindSlice<U>> FindSlice<U> for SourceLexer<T> {
    fn find_slice(&self, substr: U) -> Option<ops::Range<usize>> {
        self.data.find_slice(substr)
    }
}

impl<T: StreamIsPartial> StreamIsPartial for SourceLexer<T> {
    type PartialState = T::PartialState;

    fn complete(&mut self) -> Self::PartialState {
        self.data.complete()
    }

    fn restore_partial(&mut self, state: Self::PartialState) {
        self.data.restore_partial(state);
    }

    fn is_partial_supported() -> bool {
        T::is_partial_supported()
    }
}

impl<'i> Stream for SourceLexer<&'i str> {
    type Token = char;
    type Slice = <&'i str as Stream>::Slice;
    type IterOffsets = <&'i str as Stream>::IterOffsets;
    type Checkpoint = SourceLexer<&'i str>;

    fn iter_offsets(&self) -> Self::IterOffsets {
        self.data.iter_offsets()
    }

    fn eof_offset(&self) -> usize {
        self.data.eof_offset()
    }

    fn next_token(&mut self) -> Option<Self::Token> {
        let next = self.data.peek_token()?;

        self.data = &self.data[next.len()..];
        self.offset += 1;

        if next == '\n' {
            self.line += 1;
            self.last_col = self.col;
            self.col = 0;
        } else {
            self.col += 1;
        }

        Some(next)
    }

    fn peek_token(&self) -> Option<Self::Token> {
        self.data.peek_token()
    }

    fn offset_for<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Token) -> bool,
    {
        self.data.offset_for(predicate)
    }

    fn offset_at(&self, tokens: usize) -> Result<usize, winnow::error::Needed> {
        self.data.offset_at(tokens)
    }

    fn next_slice(&mut self, offset: usize) -> Self::Slice {
        let slice = self.data.next_slice(offset);

        for token in slice.chars() {
            self.offset += 1;
            if token == '\n' {
                self.line += 1;
                self.last_col = self.col;
                self.col = 0;
            } else {
                self.col += 1;
            }
        }

        slice
    }

    fn peek_slice(&self, offset: usize) -> Self::Slice {
        self.data.peek_slice(offset)
    }

    fn checkpoint(&self) -> Self::Checkpoint {
        self.clone()
    }

    fn reset(&mut self, checkpoint: &Self::Checkpoint) {
        *self = checkpoint.clone();
    }

    fn raw(&self) -> &dyn fmt::Debug {
        self.data.raw()
    }
}

impl<'i> ParserError<SourceLexer<&'i str>> for LexerError {
    type Inner = Self;

    fn from_input(input: &SourceLexer<&'i str>) -> Self {
        Self {
            span: input.span(),
            labels: Vec::new(),
            backtrace: Backtrace::force_capture(),
        }
    }

    fn into_inner(self) -> winnow::Result<Self::Inner, Self> {
        Ok(self)
    }
}

impl<'i, C: ToString> AddContext<SourceLexer<&'i str>, C> for LexerError {
    fn add_context(
        self,
        input: &SourceLexer<&'i str>,
        _token_start: &SourceLexer<&'i str>,
        context: C,
    ) -> Self {
        let mut labels = self.labels.clone();
        let span = self.span.range_to(input.span());
        labels.push((
            span,
            ctx_line(&input.base, span.from.offset).to_owned(),
            context.to_string(),
        ));

        Self {
            span: self.span,
            labels,
            backtrace: self.backtrace,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.labels.is_empty() {
            writeln!(f, "\x1b[31;1mUnexpected error at {:#?}", self.span)?;
        } else {
            for (span, ctx, msg) in &self.labels {
                let line_align = span.to.line.to_string().len();

                let cursor_offset = " ".repeat(span.from.col);
                let cursor = "~".repeat(span.to.col - span.from.col + 1);

                writeln!(f, "\x1b[31merror: \x1b[1m{msg}\x1b[0m")?;
                writeln!(f, "\x1b[36m {} \x1b[34m| \x1b[0m{ctx}", span.from.line)?;
                writeln!(
                    f,
                    "\x1b[36m {:line_align$} \x1b[34m| \x1b[31m{cursor_offset}{cursor}\x1b[0m",
                    ""
                )?;
            }
        }

        let backtrace = self.backtrace.to_string();

        let mut backtrace = backtrace.split('\n');

        writeln!(f, "backtrace:")?;
        while let Some(line) = backtrace.next() {
            let Some(colon_idx) = line.find(':') else {
                break;
            };

            let module = &line[colon_idx + 2..];
            let Some(file_line) = backtrace.next() else {
                break;
            };

            if module.starts_with("std") || module.starts_with("core") {
                continue;
            }

            let Some(at_idx) = file_line.find("at ") else {
                break;
            };

            let file = &file_line[at_idx + 3..];

            if !file.starts_with(".") {
                continue;
            }

            writeln!(f, "  \x1b[31mat \x1b[33m{file} \x1b[31m({module})\x1b[0m")?;
        }

        Ok(())
    }
}
