pub mod node;
mod pretty_print;
pub mod source;
#[cfg(test)]
mod tests;
pub mod utils;

use std::collections::VecDeque;

use node::{AstBinaryOp, AstExpr, AstScope, AstStatement, AstUnaryOp};
use source::SourceAst;

use crate::lexer::token::{SpannedToken, Token};
use crate::{T, kw, scope};

impl AstScope {
    pub fn from_tokens(base: &str, tokens: VecDeque<SpannedToken>) -> AstScope {
        SourceAst::new(base, tokens).parse_scope(0)
    }
}

macro_rules! fn_bin_op {
    ($fn:ident, $base:ident; $($tk:ident => $op:ident),+  ) => {
        fn $fn(&mut self) -> AstExpr {
            let Some(mut tokens) = self
                .tokens
                .iter()
                .take_while(|t| **t != T![Newline])
                .position(|t| false $(|| *t == T![$tk])+)
                .map(|idx| self.tokens.split_off(idx + 1))
                .map(|tokens| self.with(tokens))
            else {
                return self.$base();
            };

            let op = self.tokens.pop_back().unwrap();

            let left = self.$base().into();

            let op = match *op {
                $(T![$tk] => AstBinaryOp::$op,)+
                _ => unreachable!(),
            };

            AstExpr::BinaryOp {
                op,
                left,
                right: tokens.$fn().into(),
            }
        }
    };
}

impl<'i> SourceAst<'i> {
    fn parse_scope(&mut self, level: usize) -> AstScope {
        let mut nodes = Vec::new();

        loop {
            if !self.parse_pre_statement(level) {
                break;
            }

            let stmt = self.parse_statement(level);

            nodes.push(stmt);
        }

        AstScope(nodes)
    }

    fn eat_indent(&mut self, level: usize) -> Option<bool> {
        for _ in 0..level {
            let tk = self.peek()?;

            match *tk.token {
                T![Indentation] => continue,
                // Next line
                T![Newline] => return Some(true),
                // Exit from scope
                _ => {
                    tk.recover();
                    return None;
                }
            }
        }

        Some(false)
    }

    /// Prepare for statement. Returns false if there're no relevant tokens at same level
    fn parse_pre_statement(&mut self, level: usize) -> bool {
        let Some(first) = self.peek() else {
            return false;
        };

        if *first != T![Newline] {
            first.recover();
            return true;
        }

        loop {
            if level != 0 {
                match self.eat_indent(level) {
                    Some(false) => {}
                    // Next line
                    Some(true) => continue,
                    None => break false,
                }
            }

            let Some(first) = self.peek() else {
                break false;
            };

            if *first == T![Newline] {
                continue;
            }

            first.recover();
            break true;
        }
    }

    fn parse_statement(&mut self, level: usize) -> AstStatement {
        let first = self.peek_expect();

        let stmt = match **first {
            Token::Ident(_) => {
                let token = first.source.peek_expect();

                if let T![Equal] = *token.token {
                    let var = first
                        .accept()
                        .token
                        .into_ident()
                        .expect("Already checked above");
                    AstStatement::VariableDeclaration(var, self.parse_expr().into())
                } else {
                    token.recover();
                    first.recover();

                    AstStatement::Expresion(self.parse_expr().into())
                }
            }

            kw!(Global) => {
                let mut vars = Vec::new();

                loop {
                    let token = self.expect_match("Ident", |t| t.token.into_ident());

                    vars.push(token);

                    let Some(token) = self.tokens.pop_front() else {
                        break
                    };

                    match *token {
                        T![Newline] => break,
                        T![Comma] => continue,
                        _ => self.error_at(
                            token.span,
                            format!("Unexpected token: {:?}. Expected ','", token.token),
                        ),
                    }
                }

                AstStatement::Global(vars)
            }

            kw!(If) => Self::parse_stmt_if(self, level),

            T![Bang] | Token::Literal(_) => {
                first.recover();
                AstStatement::Expresion(self.parse_expr().into())
            }

            _ => {
                let first = first.accept();
                self.error_at(first.span, format!("Unexpected token: {:?}.", first.token))
            }
        };

        stmt
    }

    fn parse_stmt_if(&mut self, level: usize) -> AstStatement {
        let test = self.parse_expr().into();

        self.expect_token(T![Colon]);

        let body = Self::parse_scope(self, level + 1);

        let mut peek_source = self.clone();

        let otherwise = Self::parse_pre_statement(&mut peek_source, level)
            .then(|| peek_source.tokens.pop_front())
            .flatten()
            .map(|keyword| match keyword.token {
                kw!(Else) => {
                    peek_source.expect_token(T![Colon]);

                    Some(Self::parse_scope(&mut peek_source, level + 1))
                }
                kw!(Elif) => Some(scope![Self::parse_stmt_if(&mut peek_source, level)]),
                _ => None,
            })
            // Update global source state if found something
            .inspect(|_| *self = peek_source)
            .flatten();

        AstStatement::Conditional {
            test,
            body,
            otherwise,
        }
    }

    fn parse_expr(&mut self) -> AstExpr {
        self.parse_expr_bin_2()
    }

    fn parse_expr_base(&mut self) -> AstExpr {
        let first = self.expect();

        match first.token {
            Token::Ident(ident) => AstExpr::Ident(ident),
            Token::Literal(lit) => AstExpr::Literal(lit),
            T![Bang] => AstExpr::UnaryOp {
                op: AstUnaryOp::Not,
                right: self.parse_expr_base().into(),
            },
            _ => self.error_at(
                first.span,
                format!("Unexpected token: {:?}. Expected expression.", first.token),
            ),
        }
    }

    fn_bin_op!{parse_expr_bin_1, parse_expr_base; Star => Mul, Slash => Div}
    fn_bin_op!{parse_expr_bin_2, parse_expr_bin_1; Add => Add, Minus => Sub}
}
