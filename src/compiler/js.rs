use core::fmt;

use crate::ast::node::{AstExpr, AstScope};
use crate::lexer::token::TokenLiteral;

use super::Compiler;

pub struct JsCompiler;

impl<W: fmt::Write> Compiler<W> for JsCompiler {
    fn visit_scope(&mut self, buf: &mut W, scope: AstScope) -> fmt::Result {
        for stmt in scope.0 {
            self.visit_stmt(buf, stmt)?;
            buf.write_char(';')?;
            buf.write_char('\n')?;
        }

        Ok(())
    }

    fn visit_expr(&mut self, buf: &mut W, expr: AstExpr) -> fmt::Result {
        match expr {
            AstExpr::BinaryOp { op, left, right } => {
                self.visit_expr(buf, *left)?;
                buf.write_fmt(format_args!(" {op} "))?;
                self.visit_expr(buf, *right)
            }
            AstExpr::Ident(var) => buf.write_str(&var),
            AstExpr::Literal(literal) => self.visit_literal(buf, literal),
            AstExpr::UnaryOp { op, right } => {
                buf.write_fmt(format_args!("{op} "))?;
                self.visit_expr(buf, *right)
            }
        }
    }

    fn visit_global(&mut self, _buf: &mut W, _vars: Vec<String>) -> fmt::Result {
        Ok(())
    }

    fn visit_var(&mut self, buf: &mut W, var: String, expr: AstExpr) -> fmt::Result {
        buf.write_str("let ")?;
        buf.write_str(&var)?;
        buf.write_str(" = ")?;
        self.visit_expr(buf, expr)
    }

    fn visit_literal(&mut self, buf: &mut W, literal: TokenLiteral) -> fmt::Result {
        match literal {
            TokenLiteral::Bool(true) => buf.write_str("true"),
            TokenLiteral::Bool(false) => buf.write_str("false"),
            TokenLiteral::Number(n) => buf.write_fmt(format_args!("{n}")),
            TokenLiteral::String(s) => buf.write_fmt(format_args!("{s:?}")),
        }
    }
}
