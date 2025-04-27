mod pretty_print;
pub mod source;
pub mod span;
#[cfg(test)]
mod tests;
pub mod token;
pub mod utils;

use std::collections::VecDeque;

use source::{LexerError, SourceLexer};
use span::Span;
use token::{SpannedToken, Token, TokenKeyword, TokenLiteral, TokenPunctuation};
use utils::eat_spaces;
use winnow::Parser;
use winnow::combinator::peek;
use winnow::stream::AsChar;
use winnow::token::{any, take, take_until, take_while};

pub type LexerResult<T = ()> = core::result::Result<T, LexerError>;

pub struct Lexer;

impl Lexer {
    pub fn from_str(input: &str) -> LexerResult<VecDeque<SpannedToken>> {
        let mut input = SourceLexer::new(input);
        let mut tokens = VecDeque::new();

        while Self::next_token(&mut tokens, &mut input).unwrap_or_else(|err| {
            print!("{err}");
            std::process::exit(1)
        }) {}

        Ok(tokens)
    }

    fn next_token(
        tokens: &mut VecDeque<SpannedToken>,
        input: &mut SourceLexer<&str>,
    ) -> LexerResult<bool> {
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

        // Eat the peeked char
        let span = input.span();
        _ = any::<_, ()>(input);

        // Starts with double quote
        if char == '"' {
            Self::token_string(span, tokens, input)?;
            return Ok(true);
        }

        let token = match char {
            '+' => TokenPunctuation::Add,
            ':' => TokenPunctuation::Colon,
            ',' => TokenPunctuation::Comma,
            ' ' => TokenPunctuation::Indentation,
            '-' => TokenPunctuation::Minus,
            '\n' => TokenPunctuation::Newline,
            '(' => TokenPunctuation::ParenOpen,
            ')' => TokenPunctuation::ParenClose,
            '#' => TokenPunctuation::Pound,
            '/' => TokenPunctuation::Slash,
            '*' => TokenPunctuation::Star,

            '!' => match peek(any::<_, ()>).parse_next(input) {
                Ok('=') => {
                    _ = any::<_, ()>(input);
                    TokenPunctuation::BangEqual
                }
                _ => TokenPunctuation::Bang,
            },
            '=' => match peek(any::<_, ()>).parse_next(input) {
                Ok('=') => {
                    _ = any::<_, ()>(input);
                    TokenPunctuation::EqualEqual
                }
                _ => TokenPunctuation::Equal,
            },
            _ => input.error(format!("Unexpected char: {char:#?}")),
        };

        let span = span.range_to(input.span());

        // Don't eat indentation
        if token != TokenPunctuation::Newline {
            eat_spaces(input)?;
        }

        tokens.push_back(SpannedToken {
            span,
            token: Token::Punctuation(token),
        });

        Ok(true)
    }

    fn token_ident(
        tokens: &mut VecDeque<SpannedToken>,
        input: &mut SourceLexer<&str>,
    ) -> LexerResult<()> {
        let span = input.span();
        let ident = take_while(1.., |c: char| c.is_alphanumeric() || c == '_').parse_next(input)?;
        let span = span.range_to(input.span());

        let token = match ident {
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

    fn token_number(
        tokens: &mut VecDeque<SpannedToken>,
        input: &mut SourceLexer<&str>,
    ) -> LexerResult<()> {
        let span = input.span();
        let num = take_while(1.., AsChar::is_dec_digit).parse_next(input)?;
        let span = span.range_to(input.span());

        tokens.push_back(SpannedToken {
            span,
            token: Token::Literal(TokenLiteral::Number(num.parse().unwrap())),
        });

        eat_spaces(input)
    }

    fn token_string(
        span: Span,
        tokens: &mut VecDeque<SpannedToken>,
        input: &mut SourceLexer<&str>,
    ) -> LexerResult<()> {
        let str = take_until(1.., "\"").parse_next(input)?;
        take(1usize).parse_next(input)?;

        let span = span.range_to(input.span());

        tokens.push_back(SpannedToken {
            span,
            token: Token::Literal(TokenLiteral::String(str.to_owned())),
        });

        eat_spaces(input)
    }
}
