use core::fmt;
use std::fmt::Write;

use crate::lexer::token::TokenLiteral;

#[derive(Debug, Clone, PartialEq)]
pub struct AstScope(pub Vec<AstStatement>);

#[derive(Debug, Clone, PartialEq)]
pub enum AstBinaryOp {
    Add,
    Div,
    Mul,
    Sub,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstExpr {
    BinaryOp {
        op: AstBinaryOp,
        left: Box<AstExpr>,
        right: Box<AstExpr>,
    },
    Ident(String),
    Literal(TokenLiteral),
    UnaryOp {
        op: AstUnaryOp,
        right: Box<AstExpr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstUnaryOp {
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstStatement {
    VariableDeclaration(String, Box<AstExpr>),
    Expresion(Box<AstExpr>),
    Global(Vec<String>),
    Conditional {
        test: Box<AstExpr>,
        body: AstScope,
        otherwise: Option<AstScope>,
    },
}

impl fmt::Display for AstBinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstBinaryOp::Add => f.write_char('+'),
            AstBinaryOp::Div => f.write_char('/'),
            AstBinaryOp::Mul => f.write_char('*'),
            AstBinaryOp::Sub => f.write_char('-'),
        }
    }
}

impl fmt::Display for AstUnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstUnaryOp::Not => f.write_char('!'),
        }
    }
}
