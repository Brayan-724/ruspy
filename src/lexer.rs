mod pretty_print;
pub mod source;
pub mod span;
#[cfg(test)]
mod tests;
pub mod token;
pub mod utils;

use std::collections::VecDeque;

use ariadne::{Color, Fmt};
use source::{LexerResult, SourceLexer, SourceLexerExt};
use span::{IntoSpan, Span};
use token::{SpannedToken, Token, TokenKeyword, TokenLiteral, TokenPunctuation};
use utils::eat_spaces;
use winnow::Parser;
use winnow::combinator::{alt, delimited, peek};
use winnow::stream::AsChar;
use winnow::token::{any, take_while};

pub struct Lexer;

impl Lexer {
    pub fn from_str<'i>(input: &'i str) -> LexerResult<'i, VecDeque<SpannedToken>> {
        let mut input = SourceLexer::new(input);
        let mut tokens = VecDeque::new();

        while Self::next_token(&mut tokens, &mut input).unwrap_or_else(|err| {
            print!("{err}");
            std::process::exit(1)
        }) {}

        Ok(tokens)
    }

    fn next_token<'i>(
        tokens: &mut VecDeque<SpannedToken>,
        input: &mut SourceLexer<'i>,
    ) -> LexerResult<'i, bool> {
        let Ok(char) = peek(any::<_, ()>).parse_next(input) else {
            return Ok(false);
        };

        // Starts with alphabetic character
        if char.is_alpha() || char == '_' {
            Self::token_ident(tokens, input)?;
            return Ok(true);
        }

        // Starts with a number
        if char.is_numeric() {
            Self::token_number(tokens, input)?;
            return Ok(true);
        }

        // Starts with double quote
        if char == '"' {
            Self::token_string(tokens, input)?;
            return Ok(true);
        }

        // Eat the peeked char
        let (token, span) = alt([
            "!=".value(TokenPunctuation::BangEqual),
            "!".value(TokenPunctuation::Bang),
            ":".value(TokenPunctuation::Colon),
            ",".value(TokenPunctuation::Comma),
            "==".value(TokenPunctuation::EqualEqual),
            "=".value(TokenPunctuation::Equal),
            "  ".value(TokenPunctuation::Indentation),
            "-".value(TokenPunctuation::Minus),
            "\n".value(TokenPunctuation::Newline),
            "+".value(TokenPunctuation::Plus),
            "/".value(TokenPunctuation::Slash),
            "*".value(TokenPunctuation::Star),
        ])
        .with_span()
        .map(|(tk, span)| (tk, Span::from(span)))
        .parse_next(input)
        .unwrap_or_else(|_: ()| {
            input.error(format!(
                "Unexpected char: {}",
                format!("{char:#?}").fg(Color::BrightRed)
            ))
        });

        // Don't eat indentation
        if token != TokenPunctuation::Newline && token != TokenPunctuation::Indentation {
            eat_spaces(input)?;
        }

        tokens.push_back(SpannedToken {
            span,
            token: Token::Punctuation(token),
        });

        Ok(true)
    }

    fn token_ident<'i>(
        tokens: &mut VecDeque<SpannedToken>,
        input: &mut SourceLexer<'i>,
    ) -> LexerResult<'i> {
        let (ident, span) = take_while(1.., |c: char| c.is_alphanumeric() || c == '_')
            .with_span()
            .map(IntoSpan::into_span)
            .parse_next(input)?;

        let token = match ident {
            "nil" => Token::Literal(TokenLiteral::Nil),
            "True" => Token::Literal(TokenLiteral::Bool(true)),
            "False" => Token::Literal(TokenLiteral::Bool(false)),

            "elif" => Token::Keyword(TokenKeyword::Elif),
            "else" => Token::Keyword(TokenKeyword::Else),
            "global" => Token::Keyword(TokenKeyword::Global),
            "if" => Token::Keyword(TokenKeyword::If),

            _ => Token::Ident(ident.to_owned()),
        };

        tokens.push_back(SpannedToken { span, token });

        eat_spaces(input)
    }

    fn token_number<'i>(
        tokens: &mut VecDeque<SpannedToken>,
        input: &mut SourceLexer<'i>,
    ) -> LexerResult<'i> {
        let (num, span) = take_while(1.., AsChar::is_dec_digit)
            .with_span()
            .map(IntoSpan::into_span)
            .parse_next(input)?;

        tokens.push_back(SpannedToken {
            span,
            token: Token::Literal(TokenLiteral::Number(num.parse().unwrap())),
        });

        eat_spaces(input)
    }

    fn token_string<'i>(
        tokens: &mut VecDeque<SpannedToken>,
        input: &mut SourceLexer<'i>,
    ) -> LexerResult<'i> {
        let (str, span) = delimited('"', take_while(0.., |c| c != '"'), '"')
            .with_span()
            .map(IntoSpan::into_span)
            .parse_next(input)?;

        tokens.push_back(SpannedToken {
            span,
            token: Token::Literal(TokenLiteral::String(str.to_owned())),
        });

        eat_spaces(input)
    }
}
