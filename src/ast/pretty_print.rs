use core::fmt;

use crate::pretty_print::*;

use super::node::{AstExpr, AstScope, AstStatement};

impl fmt::Display for AstScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = f.width().unwrap_or(0);

        for stmt in &self.0 {
            f.write_fmt(format_args!("{stmt:level$}\n"))?;
        }

        Ok(())
    }
}

impl fmt::Display for AstStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = f.width().map_or(0, |l| l + 1);

        let indent = " ".repeat(level);

        f.write_str(&indent)?;

        match self {
            AstStatement::VariableDeclaration(var, expr) => {
                f.write_fmt(format_args!("{VARIABLE}{var}{R} {PUNCTUATION}={R} {expr}"))
            }
            AstStatement::Expresion(expr) => f.write_fmt(format_args!("{expr}")),
            AstStatement::Global(vec) => f.write_fmt(format_args!("{KEYWORD}global{R} {vec:?}")),
            AstStatement::Conditional {
                test,
                body,
                otherwise: None,
            } => f.write_fmt(format_args!(
                "{KEYWORD}if{R} {test}{PUNCTUATION}:{R}\n{body:level$}"
            )),
            AstStatement::Conditional {
                test,
                body,
                otherwise: Some(otherwise),
            } => f.write_fmt(format_args!(
                "{KEYWORD}if{R} {test}{PUNCTUATION}:{R}\n{body:level$}{indent}{KEYWORD}else{R}{PUNCTUATION}:{R}\n{otherwise:level$}"
            )),
        }
    }
}

impl fmt::Display for AstExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstExpr::BinaryOp { op, left, right } => {
                f.write_fmt(format_args!("{left} {PUNCTUATION}{op}{R} {right}"))
            }
            AstExpr::Ident(ident) => f.write_fmt(format_args!("{VARIABLE}{ident}{R}")),
            AstExpr::Literal(token_literal) => f.write_fmt(format_args!("{token_literal}")),
            AstExpr::UnaryOp { op, right } => {
                f.write_fmt(format_args!("{PUNCTUATION}{op}{R} {right}"))
            }
        }
    }
}
