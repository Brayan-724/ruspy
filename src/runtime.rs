#[cfg(test)]
mod tests;
pub mod value;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use value::{AsNumber, AsString, RuntimeValue, RuntimeVariable};

use crate::ast::node::{AstBinaryOp, AstExpr, AstScope, AstStatement, AstUnaryOp};
use crate::lexer::token::TokenLiteral;

#[derive(Debug)]
pub struct Scope {
    variables: Rc<RefCell<HashMap<String, RuntimeVariable>>>,
    is_function: bool,
    parent: Option<Rc<Scope>>,
}

impl Scope {
    pub fn new() -> Rc<Scope> {
        Scope {
            is_function: false,
            variables: Rc::new(RefCell::new(HashMap::new())),
            parent: None,
        }
        .into()
    }
    pub fn child(self: &Rc<Self>, is_function: bool) -> Rc<Self> {
        Scope {
            is_function,
            variables: Rc::new(RefCell::new(HashMap::new())),
            parent: Some(self.clone()),
        }
        .into()
    }

    pub fn run(self: &Rc<Self>, ast: AstScope) {
        for stmt in ast.0 {
            self.visit_stmt(stmt);
        }
    }

    pub fn get_variable(self: &Rc<Self>, name: &String) -> Option<RuntimeVariable> {
        self.variables
            .borrow()
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref().and_then(|p| p.get_variable(name)))
    }

    pub fn set_variable(self: &Rc<Self>, name: String, value: RuntimeValue) -> RuntimeVariable {
        self.set_variable_inner(name, value)
    }

    fn set_variable_inner(self: &Rc<Self>, name: String, value: RuntimeValue) -> RuntimeVariable {
        let Some(var) = self.variables.borrow().get(&name).cloned() else {
            // Functions keep a secure context
            // to manipulate external variables
            // use `global` keyword, that clones
            // variables to current scope
            if self.is_function {
                return self
                    .variables
                    .borrow_mut()
                    .entry(name)
                    .insert_entry(value.into())
                    .get()
                    .clone();
            }

            let Some(parent) = self.parent.as_ref() else {
                return self
                    .variables
                    .borrow_mut()
                    .entry(name)
                    .insert_entry(value.into())
                    .get()
                    .clone();
            };

            return parent.set_variable_inner(name, value);
        };

        // SAFETY: This function is always called with Some
        // but needs option to know if someone use the value
        // and keep the ownership if not
        *var.0.borrow_mut() = value;

        var
    }

    pub fn visit_stmt(self: &Rc<Self>, stmt: AstStatement) {
        match stmt {
            AstStatement::Conditional {
                test,
                body,
                otherwise,
            } => self.visit_conditional(*test, body, otherwise),
            AstStatement::Expresion(expr) => {
                self.visit_expr(*expr);
            }
            AstStatement::Global(vars) => self.visit_global(vars),
            AstStatement::VariableDeclaration(var, expr) => self.visit_var_decl(var, *expr),
        }
    }

    pub fn visit_conditional(self: &Rc<Self>, test: AstExpr, body: AstScope, _: Option<AstScope>) {
        let test = self.visit_expr(test);

        let test = match test {
            RuntimeValue::Nil => false,
            RuntimeValue::Bool(b) => b,
            RuntimeValue::Number(n) => n != 0,
            RuntimeValue::String(s) => !s.is_empty(),
        };

        if test {
            self.run(body);
        }
    }

    pub fn visit_expr(self: &Rc<Self>, expr: AstExpr) -> RuntimeValue {
        match expr {
            AstExpr::BinaryOp { op, left, right } => self.visit_expr_binop(op, *left, *right),
            AstExpr::Ident(var) => self
                .get_variable(&var)
                .map_or_else(|| RuntimeValue::Nil, |var| var.0.borrow().clone()),
            AstExpr::Literal(TokenLiteral::Nil) => RuntimeValue::Nil,
            AstExpr::Literal(TokenLiteral::Bool(b)) => RuntimeValue::Bool(b),
            AstExpr::Literal(TokenLiteral::Number(n)) => RuntimeValue::Number(n),
            AstExpr::Literal(TokenLiteral::String(s)) => RuntimeValue::String(s),
            AstExpr::UnaryOp {
                op: AstUnaryOp::Not,
                right,
            } => match self.visit_expr(*right) {
                RuntimeValue::Nil => RuntimeValue::Bool(true),
                RuntimeValue::Bool(b) => RuntimeValue::Bool(!b),
                RuntimeValue::Number(n) => RuntimeValue::Bool(n == 0),
                RuntimeValue::String(n) => RuntimeValue::Bool(n.is_empty()),
            },
        }
    }

    pub fn visit_expr_binop(
        self: &Rc<Self>,
        op: AstBinaryOp,
        left: AstExpr,
        right: AstExpr,
    ) -> RuntimeValue {
        match (op, self.visit_expr(left), self.visit_expr(right)) {
            ////// Number Primitives //////
            (AstBinaryOp::Add, RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                RuntimeValue::Number(a + b)
            }
            (AstBinaryOp::Div, RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                RuntimeValue::Number(a / b)
            }
            (AstBinaryOp::Mul, RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                RuntimeValue::Number(a * b)
            }
            (AstBinaryOp::Sub, RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                RuntimeValue::Number(a - b)
            }

            ////// Bool "Primitives" //////
            (AstBinaryOp::Add, RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => {
                RuntimeValue::Number(a.as_num() + b.as_num())
            }
            (AstBinaryOp::Add, RuntimeValue::Bool(false), RuntimeValue::Number(n))
            | (AstBinaryOp::Add, RuntimeValue::Number(n), RuntimeValue::Bool(false)) => {
                RuntimeValue::Number(n)
            }
            (AstBinaryOp::Add, RuntimeValue::Bool(true), RuntimeValue::Number(n))
            | (AstBinaryOp::Add, RuntimeValue::Number(n), RuntimeValue::Bool(true)) => {
                RuntimeValue::Number(n + 1)
            }
            (AstBinaryOp::Div, RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => {
                RuntimeValue::Number(a.as_num() / b.as_num())
            }
            (AstBinaryOp::Div, RuntimeValue::Bool(a), RuntimeValue::Number(b)) => {
                RuntimeValue::Number(a.as_num() / b)
            }
            (AstBinaryOp::Div, RuntimeValue::Number(a), RuntimeValue::Bool(b)) => {
                RuntimeValue::Number(a / b.as_num())
            }
            (AstBinaryOp::Mul, RuntimeValue::Bool(a), RuntimeValue::Number(b)) => {
                RuntimeValue::Number(a.as_num() * b)
            }
            (AstBinaryOp::Mul, RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => {
                RuntimeValue::Number(a.as_num() * b.as_num())
            }
            (AstBinaryOp::Mul, RuntimeValue::Number(a), RuntimeValue::Bool(b)) => {
                RuntimeValue::Number(a * b.as_num())
            }
            (AstBinaryOp::Sub, RuntimeValue::Bool(a), RuntimeValue::Bool(b)) => {
                RuntimeValue::Number(a.as_num() - b.as_num())
            }
            (AstBinaryOp::Sub, RuntimeValue::Bool(a), RuntimeValue::Number(b)) => {
                RuntimeValue::Number(a.as_num() - b)
            }
            (AstBinaryOp::Sub, RuntimeValue::Number(a), RuntimeValue::Bool(b)) => {
                RuntimeValue::Number(a - b.as_num())
            }

            ////// Concatenation //////
            (AstBinaryOp::Add, RuntimeValue::String(a), RuntimeValue::Nil) => {
                RuntimeValue::String(format!("{a}nil"))
            }
            (AstBinaryOp::Add, RuntimeValue::Nil, RuntimeValue::String(b)) => {
                RuntimeValue::String(format!("nil{b}"))
            }
            (AstBinaryOp::Add, RuntimeValue::Bool(a), RuntimeValue::String(b)) => {
                RuntimeValue::String(format!("{}{b}", a.as_string()))
            }
            (AstBinaryOp::Add, RuntimeValue::String(a), RuntimeValue::Bool(b)) => {
                RuntimeValue::String(format!("{a}{}", b.as_string()))
            }
            (AstBinaryOp::Add, RuntimeValue::Number(a), RuntimeValue::String(b)) => {
                RuntimeValue::String(format!("{a}{b}"))
            }
            (AstBinaryOp::Add, RuntimeValue::String(a), RuntimeValue::Number(b)) => {
                RuntimeValue::String(format!("{a}{b}"))
            }
            (AstBinaryOp::Add, RuntimeValue::String(a), RuntimeValue::String(b)) => {
                RuntimeValue::String(format!("{a}{b}"))
            }

            ////// Multiplication //////
            (AstBinaryOp::Mul, RuntimeValue::String(s), RuntimeValue::Bool(true))
            | (AstBinaryOp::Mul, RuntimeValue::Bool(true), RuntimeValue::String(s)) => {
                RuntimeValue::String(s)
            }
            (AstBinaryOp::Mul, RuntimeValue::String(_), RuntimeValue::Bool(false))
            | (AstBinaryOp::Mul, RuntimeValue::Bool(false), RuntimeValue::String(_)) => {
                RuntimeValue::String(String::new())
            }
            (AstBinaryOp::Mul, RuntimeValue::Number(a), RuntimeValue::String(b)) => {
                RuntimeValue::String(
                    a.is_positive()
                        .then(|| b.repeat(a.unsigned_abs() as usize))
                        .unwrap_or_default(),
                )
            }
            (AstBinaryOp::Mul, RuntimeValue::String(a), RuntimeValue::Number(b)) => {
                RuntimeValue::String(
                    b.is_positive()
                        .then(|| a.repeat(b.unsigned_abs() as usize))
                        .unwrap_or_default(),
                )
            }

            (_, _, RuntimeValue::String(_) | RuntimeValue::Nil)
            | (_, RuntimeValue::String(_) | RuntimeValue::Nil, _) => RuntimeValue::Nil,
        }
    }

    pub fn visit_global(self: &Rc<Self>, vars: Vec<String>) {
        let Some(parent) = self.parent.as_ref() else {
            return;
        };

        for var in vars {
            let value = parent
                .get_variable(&var)
                .unwrap_or_else(|| parent.set_variable(var.clone(), RuntimeValue::Nil));

            self.variables.borrow_mut().insert(var, value);
        }
    }

    pub fn visit_var_decl(self: &Rc<Self>, var: String, expr: AstExpr) {
        self.set_variable(var, self.visit_expr(expr));
    }
}
