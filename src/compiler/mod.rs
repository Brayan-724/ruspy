use core::fmt;

use crate::ast::node::{AstExpr, AstScope, AstStatement};
use crate::lexer::token::TokenLiteral;

pub mod js;

pub trait Compiler<W: fmt::Write> {
    fn visit_scope(&mut self, buf: &mut W, scope: AstScope) -> fmt::Result {
        for stmt in scope.0 {
            self.visit_stmt(buf, stmt)?;
        }
        Ok(())
    }
    fn visit_stmt(&mut self, buf: &mut W, stmt: AstStatement) -> fmt::Result {
        match stmt {
            AstStatement::Conditional { .. } => todo!(),
            AstStatement::Expresion(expr) => self.visit_expr(buf, *expr),
            AstStatement::Global(vars) => self.visit_global(buf, vars),
            AstStatement::VariableDeclaration(var, expr) => self.visit_var(buf, var, *expr),
        }
    }
    fn visit_expr(&mut self, buf: &mut W, expr: AstExpr) -> fmt::Result;
    fn visit_global(&mut self, buf: &mut W, vars: Vec<String>) -> fmt::Result;
    fn visit_var(&mut self, buf: &mut W, var: String, expr: AstExpr) -> fmt::Result;
    fn visit_literal(&mut self, buf: &mut W, literal: TokenLiteral) -> fmt::Result;
}
