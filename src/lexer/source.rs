use core::fmt;
use std::backtrace::Backtrace;

use ariadne::{Label, Report, ReportKind, Source};
use winnow::LocatingSlice;
use winnow::error::{AddContext, ParserError};
use winnow::stream::{Location, Stream};

use super::span::Span;

pub type SourceLexer<'i> = LocatingSlice<&'i str>;
pub type LexerResult<'i, T = ()> = Result<T, LexerError<'i>>;

#[derive(Debug)]
pub struct LexerError<'i> {
    base: &'i str,
    span: Span,
    labels: Vec<(Span, String)>,
    backtrace: Backtrace,
}

pub trait SourceLexerExt<'i> {
    fn base(&self) -> &'i str;
    fn span(&self) -> Span;
    fn error(&self, msg: impl fmt::Display) -> !;
}

impl<'i> SourceLexerExt<'i> for SourceLexer<'i> {
    fn base(&self) -> &'i str {
        let mut base = *self;
        base.reset_to_start();
        *base
    }

    fn span(&self) -> Span {
        Span::char(self.current_token_start())
    }

    fn error(&self, msg: impl fmt::Display) -> ! {
        _ = Report::build(ReportKind::Error, self.span())
            .with_message(&msg)
            .with_label(
                Label::new(self.span())
                    .with_message(msg)
                    .with_color(ariadne::Color::BrightRed),
            )
            .finish()
            .eprint(Source::from(self.base()));

        std::process::exit(1);
    }
}

impl<'i> ParserError<SourceLexer<'i>> for LexerError<'i> {
    type Inner = Self;

    fn from_input(input: &SourceLexer<'i>) -> Self {
        Self {
            base: input.base(),
            span: input.span(),
            labels: Vec::new(),
            backtrace: Backtrace::force_capture(),
        }
    }

    fn into_inner(self) -> winnow::Result<Self::Inner, Self> {
        Ok(self)
    }
}

impl<'i, C: ToString> AddContext<SourceLexer<'i>, C> for LexerError<'i> {
    fn add_context(
        mut self,
        input: &SourceLexer<'i>,
        _token_start: &<SourceLexer<'i> as Stream>::Checkpoint,
        context: C,
    ) -> Self {
        self.labels.push((input.span(), context.to_string()));
        self
    }
}

impl fmt::Display for LexerError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        _ = Report::build(ReportKind::Error, self.span)
            .with_labels(
                self.labels
                    .iter()
                    .map(|(span, msg)| Label::new(*span).with_message(msg)),
            )
            .finish()
            .eprint(Source::from(self.base));

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

            if !file.starts_with('.') {
                continue;
            }

            writeln!(f, "  \x1b[31mat \x1b[33m{file} \x1b[31m({module})\x1b[0m")?;
        }

        Ok(())
    }
}
