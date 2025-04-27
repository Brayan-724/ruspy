pub mod node;
mod pretty_print;
pub mod source;
#[cfg(test)]
mod tests;
pub mod utils;

use std::collections::VecDeque;

use node::{AstBinaryOp, AstExpr, AstScope, AstStatement, AstUnaryOp};

use crate::lexer::token::{SpannedToken, Token};
use crate::{T, kw, scope};

impl AstScope {
    pub fn from_tokens(mut tokens: VecDeque<SpannedToken>) -> AstScope {
        Self::parse_scope(&mut tokens, 0)
    }

    fn expect(tokens: &mut VecDeque<SpannedToken>) -> SpannedToken {
        tokens
            .pop_front()
            .unwrap_or_else(|| panic!("Unexpected token: EOF"))
    }

    fn expect_token(tokens: &mut VecDeque<SpannedToken>, token: Token) {
        if let Some(first) = tokens.pop_front() {
            if first != token {
                panic!("Unexpected token: {first:?}. Expected: {token:#?}")
            }
        } else {
            panic!("Unexpected token: EOF. Expected: {token:#?}")
        }
    }

    fn parse_scope(tokens: &mut VecDeque<SpannedToken>, level: usize) -> AstScope {
        let mut nodes = Vec::new();

        loop {
            if !Self::parse_pre_statement(tokens, level) {
                break;
            }

            let stmt = Self::parse_statement(tokens, level);

            nodes.push(stmt);
        }

        AstScope(nodes)
    }

    /// Prepare for statement. Returns false if there're no relevant tokens at same level
    fn parse_pre_statement(tokens: &mut VecDeque<SpannedToken>, level: usize) -> bool {
        let Some(first) = tokens.pop_front() else {
            return false;
        };
        let is_ln = first == T![Newline];

        if !is_ln {
            tokens.push_front(first);
            return true;
        }

        'next_line: loop {
            if level != 0 {
                for _ in 0..level {
                    match tokens.pop_front() {
                        Some(SpannedToken {
                            token: T![Newline], ..
                        }) => continue 'next_line,
                        Some(SpannedToken {
                            token: T![Indentation],
                            ..
                        }) => {}
                        // Exit from scope
                        Some(tk) => {
                            tokens.push_front(tk);
                            return false;
                        }
                        None => return false,
                    }
                }
            }

            let Some(first) = tokens.pop_front() else {
                return false;
            };

            if first == T![Newline] {
                continue;
            }

            tokens.push_front(first);
            return true;
        }
    }

    fn parse_statement(tokens: &mut VecDeque<SpannedToken>, level: usize) -> AstStatement {
        let first = unsafe { tokens.pop_front().unwrap_unchecked() };

        let (first_span, first) = first.parts();
        let stmt = match first {
            Token::Ident(var) => {
                let token = Self::expect(tokens);

                if let T![Equal] = token.token {
                    AstStatement::VariableDeclaration(var, Box::new(Self::parse_expr(tokens)))
                } else {
                    tokens.push_front(token);
                    tokens.push_front(SpannedToken::new(first_span, Token::Ident(var)));

                    AstStatement::Expresion(Box::new(Self::parse_expr(tokens)))
                }
            }
            kw!(Global) => {
                let mut vars = Vec::new();

                loop {
                    let token = tokens.pop_front().expect("Unexpected eof");

                    if let Token::Ident(ident) = token.token {
                        vars.push(ident)
                    } else {
                        panic!("Unexpected token: {token:?}. Expected ident")
                    }

                    let token = tokens.pop_front();

                    match token {
                        None
                        | Some(SpannedToken {
                            token: T![Newline], ..
                        }) => break,
                        Some(SpannedToken {
                            token: T![Comma], ..
                        }) => continue,
                        _ => panic!("Unexpected token: {token:?}. Expected ','"),
                    }
                }

                AstStatement::Global(vars)
            }
            kw!(If) => Self::parse_stmt_if(tokens, level),
            Token::Literal(_) => {
                tokens.push_front(SpannedToken::new(first_span, first));
                AstStatement::Expresion(Box::new(Self::parse_expr(tokens)))
            }
            T![Bang] => {
                tokens.push_front(SpannedToken::new(first_span, first));
                AstStatement::Expresion(Box::new(Self::parse_expr(tokens)))
            }

            _ => panic!("Unexpected token: {first:?}."),
        };

        stmt
    }

    fn parse_stmt_if(tokens: &mut VecDeque<SpannedToken>, level: usize) -> AstStatement {
        let test = Self::parse_expr(tokens).into();
        Self::expect_token(tokens, T![Colon]);

        let body = Self::parse_scope(tokens, level + 1);

        let mut peek_tokens = tokens.clone();

        let otherwise = if Self::parse_pre_statement(&mut peek_tokens, level) {
            if let Some(keyword) = peek_tokens.pop_front() {
                match keyword.token {
                    kw!(Else) => {
                        *tokens = peek_tokens;
                        Self::expect_token(tokens, T!(Colon));

                        Some(Self::parse_scope(tokens, level + 1))
                    }
                    kw!(Elif) => {
                        *tokens = peek_tokens;
                        Some(scope![Self::parse_stmt_if(tokens, level)])
                    }
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        };

        AstStatement::Conditional {
            test,
            body,
            otherwise,
        }
    }

    fn parse_expr(tokens: &mut VecDeque<SpannedToken>) -> AstExpr {
        fn expr_base(tokens: &mut VecDeque<SpannedToken>) -> AstExpr {
            let first = tokens.pop_front().expect("Unexpected eof");

            match first.token {
                Token::Ident(ident) => AstExpr::Ident(ident),
                Token::Literal(lit) => AstExpr::Literal(lit),
                T![Bang] => AstExpr::UnaryOp {
                    op: AstUnaryOp::Not,
                    right: expr_base(tokens).into(),
                },
                _ => panic!("Unexpected token: {first:?}. Expected expression."),
            }
        }

        fn bin_op_mul_div(left_tokens: &mut VecDeque<SpannedToken>) -> AstExpr {
            let Some(mut tokens) = left_tokens
                .iter()
                .take_while(|t| **t != T![Newline])
                .position(|t| *t == T![Star] || *t == T![Slash])
                .map(|idx| left_tokens.split_off(idx + 1))
            else {
                return expr_base(left_tokens);
            };

            let op = left_tokens.pop_back().expect("At least have one token");

            let op = match op.token {
                T![Star] => AstBinaryOp::Mul,
                T![Slash] => AstBinaryOp::Div,
                _ => unreachable!(),
            };

            AstExpr::BinaryOp {
                op,
                left: expr_base(left_tokens).into(),
                right: bin_op_mul_div(&mut tokens).into(),
            }
        }

        fn bin_op_add_sub(left_tokens: &mut VecDeque<SpannedToken>) -> AstExpr {
            let Some(mut tokens) = left_tokens
                .iter()
                .take_while(|t| **t != T![Newline])
                .position(|t| *t == T![Add] || *t == T![Minus])
                .map(|idx| left_tokens.split_off(idx + 1))
            else {
                return bin_op_mul_div(left_tokens);
            };

            let op = left_tokens.pop_back().expect("At least have one token");

            let op = match op.token {
                T![Add] => AstBinaryOp::Add,
                T![Minus] => AstBinaryOp::Sub,
                _ => unreachable!(),
            };

            AstExpr::BinaryOp {
                op,
                left: bin_op_mul_div(left_tokens).into(),
                right: bin_op_add_sub(&mut tokens).into(),
            }
        }

        bin_op_add_sub(tokens)
    }
}
