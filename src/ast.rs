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
        Self::parse_scope(&mut SourceAst::new(base, tokens), 0)
    }

    fn parse_scope(source: &mut SourceAst<'_>, level: usize) -> AstScope {
        let mut nodes = Vec::new();

        loop {
            if !Self::parse_pre_statement(source, level) {
                break;
            }

            let stmt = Self::parse_statement(source, level);

            nodes.push(stmt);
        }

        AstScope(nodes)
    }

    fn eat_indent(source: &mut SourceAst<'_>, level: usize) -> Option<bool> {
        for _ in 0..level {
            let tk = source.peek()?;

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
    fn parse_pre_statement(source: &mut SourceAst<'_>, level: usize) -> bool {
        let Some(first) = source.peek() else {
            return false;
        };

        if *first != T![Newline] {
            first.recover();
            return true;
        }

        loop {
            if level != 0 {
                match Self::eat_indent(source, level) {
                    Some(false) => {}
                    // Next line
                    Some(true) => continue,
                    None => break false,
                }
            }

            let Some(first) = source.peek() else {
                break false;
            };

            if *first == T![Newline] {
                continue;
            }

            first.recover();
            break true;
        }
    }

    fn parse_statement(source: &mut SourceAst<'_>, level: usize) -> AstStatement {
        let first = source.peek_expect();

        let stmt = match **first {
            Token::Ident(_) => {
                let token = first.source.peek_expect();

                if let T![Equal] = *token.token {
                    let var = first
                        .accept()
                        .token
                        .into_ident()
                        .expect("Already checked above");
                    AstStatement::VariableDeclaration(var, Box::new(Self::parse_expr(source)))
                } else {
                    token.recover();
                    first.recover();

                    AstStatement::Expresion(Box::new(Self::parse_expr(source)))
                }
            }
            kw!(Global) => {
                let mut vars = Vec::new();

                loop {
                    let token = source.expect_match("Ident", |t| t.token.into_ident());

                    vars.push(token);

                    let token = source.tokens.pop_front();

                    match token {
                        None
                        | Some(SpannedToken {
                            token: T![Newline], ..
                        }) => break,
                        Some(SpannedToken {
                            token: T![Comma], ..
                        }) => continue,
                        Some(token) => source.error_at(
                            token.span,
                            format!("Unexpected token: {:?}. Expected ','", token.token),
                        ),
                    }
                }

                AstStatement::Global(vars)
            }
            kw!(If) => Self::parse_stmt_if(source, level),
            Token::Literal(_) => {
                first.recover();
                AstStatement::Expresion(Box::new(Self::parse_expr(source)))
            }
            T![Bang] => {
                first.recover();
                AstStatement::Expresion(Box::new(Self::parse_expr(source)))
            }

            _ => {
                let first = first.accept();
                source.error_at(first.span, format!("Unexpected token: {:?}.", first.token))
            }
        };

        stmt
    }

    fn parse_stmt_if(source: &mut SourceAst<'_>, level: usize) -> AstStatement {
        let test = Self::parse_expr(source).into();

        source.expect_token(T![Colon]);

        let body = Self::parse_scope(source, level + 1);

        let mut peek_source = source.clone();

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
            .inspect(|_| *source = peek_source)
            .flatten();

        AstStatement::Conditional {
            test,
            body,
            otherwise,
        }
    }

    fn parse_expr(source: &mut SourceAst<'_>) -> AstExpr {
        fn expr_base(source: &mut SourceAst<'_>) -> AstExpr {
            let first = source.expect();

            match first.token {
                Token::Ident(ident) => AstExpr::Ident(ident),
                Token::Literal(lit) => AstExpr::Literal(lit),
                T![Bang] => AstExpr::UnaryOp {
                    op: AstUnaryOp::Not,
                    right: expr_base(source).into(),
                },
                _ => source.error_at(
                    first.span,
                    format!("Unexpected token: {:?}. Expected expression.", first.token),
                ),
            }
        }

        fn bin_op_mul_div(left_source: &mut SourceAst<'_>) -> AstExpr {
            let Some(mut tokens) = left_source
                .tokens
                .iter()
                .take_while(|t| **t != T![Newline])
                .position(|t| *t == T![Star] || *t == T![Slash])
                .map(|idx| left_source.tokens.split_off(idx + 1))
                .map(|tokens| left_source.with(tokens))
            else {
                return expr_base(left_source);
            };

            let op = left_source.expect();

            let op = match op.token {
                T![Star] => AstBinaryOp::Mul,
                T![Slash] => AstBinaryOp::Div,
                _ => unreachable!(),
            };

            AstExpr::BinaryOp {
                op,
                left: expr_base(left_source).into(),
                right: bin_op_mul_div(&mut tokens).into(),
            }
        }

        fn bin_op_add_sub(left_source: &mut SourceAst<'_>) -> AstExpr {
            let Some(mut tokens) = left_source
                .tokens
                .iter()
                .take_while(|t| **t != T![Newline])
                .position(|t| *t == T![Add] || *t == T![Minus])
                .map(|idx| left_source.tokens.split_off(idx + 1))
                .map(|tokens| left_source.with(tokens))
            else {
                return bin_op_mul_div(left_source);
            };

            let op = left_source.expect();

            let op = match op.token {
                T![Add] => AstBinaryOp::Add,
                T![Minus] => AstBinaryOp::Sub,
                _ => unreachable!(),
            };

            AstExpr::BinaryOp {
                op,
                left: bin_op_mul_div(left_source).into(),
                right: bin_op_add_sub(&mut tokens).into(),
            }
        }

        bin_op_add_sub(source)
    }
}
