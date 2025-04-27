#[cfg(test)]
mod tests;
pub mod value;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use value::{RuntimeValue, RuntimeVariable};

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

    pub fn set_variable(self: &Rc<Self>, name: String, value: RuntimeValue) {
        let mut value = Some(value);

        self.set_variable_inner(&name, &mut value);

        // If no one use value, create the variable
        if let Some(value) = value {
            self.variables.borrow_mut().insert(name, value.into());
        }
    }

    fn set_variable_inner(self: &Rc<Self>, name: &String, value: &mut Option<RuntimeValue>) {
        let Some(var) = self.variables.borrow().get(name).cloned() else {
            // Functions keep a secure context
            // to manipulate external variables
            // use `global` keyword, that clones
            // variables to current scope
            if self.is_function {
                return;
            }

            let Some(parent) = self.parent.as_ref() else {
                return;
            };

            return parent.set_variable_inner(name, value);
        };

        // SAFETY: This function is always called with Some
        // but needs option to know if someone use the value
        // and keep the ownership if not
        *var.0.borrow_mut() = unsafe { value.take().unwrap_unchecked() };
    }

    pub fn visit_stmt(self: &Rc<Self>, stmt: AstStatement) {
        match stmt {
            AstStatement::Conditional {
                test,
                body,
                otherwise,
            } => self.visit_conditional(test, body, otherwise),
            AstStatement::Expresion(expr) => {
                self.visit_expr(expr);
            }
            AstStatement::Global(vars) => self.visit_global(vars),
            AstStatement::VariableDeclaration(var, expr) => self.visit_var_decl(var, expr),
        }
    }

    pub fn visit_conditional(
        self: &Rc<Self>,
        test: Box<AstExpr>,
        body: AstScope,
        _: Option<AstScope>,
    ) {
        let test = self.visit_expr(test);

        if let RuntimeValue::Bool(test) = test {
            if test {
                self.run(body);
            }
        } else {
            panic!("Conditionals use booleans, this is not Javascript")
        }
    }

    pub fn visit_expr(self: &Rc<Self>, expr: Box<AstExpr>) -> RuntimeValue {
        match *expr {
            AstExpr::BinaryOp { op, left, right } => {
                match (op, self.visit_expr(left), self.visit_expr(right)) {
                    (AstBinaryOp::Mul, RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                        RuntimeValue::Number(a * b)
                    }
                    (AstBinaryOp::Div, RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                        RuntimeValue::Number(a / b)
                    }
                    (AstBinaryOp::Sub, RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                        RuntimeValue::Number(a - b)
                    }
                    (AstBinaryOp::Add, RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                        RuntimeValue::Number(a + b)
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
                    (op, a, b) => panic!("Invalid operation types: {a:?} {op:?} {b:?}"),
                }
            }
            AstExpr::Ident(var) => self
                .get_variable(&var)
                .unwrap_or_else(|| panic!("Undefined variable: {var}"))
                .0
                .borrow()
                .clone(),
            AstExpr::Literal(TokenLiteral::Bool(b)) => RuntimeValue::Bool(b),
            AstExpr::Literal(TokenLiteral::Number(n)) => RuntimeValue::Number(n),
            AstExpr::Literal(TokenLiteral::String(s)) => RuntimeValue::String(s),
            AstExpr::UnaryOp {
                op: AstUnaryOp::Not,
                right,
            } => match self.visit_expr(right) {
                RuntimeValue::Bool(b) => RuntimeValue::Bool(b),
                v => panic!("Unsupported type: {v:?}"),
            },
        }
    }

    pub fn visit_global(self: &Rc<Self>, vars: Vec<String>) {
        let Some(parent) = self.parent.as_ref() else {
            return;
        };

        for var in vars {
            let Some(value) = parent.get_variable(&var) else {
                panic!("Trying to globalize undeclared variable: {var}")
            };

            self.variables.borrow_mut().insert(var, value);
        }
    }

    pub fn visit_var_decl(self: &Rc<Self>, var: String, expr: Box<AstExpr>) {
        self.set_variable(var, self.visit_expr(expr));
    }
}
