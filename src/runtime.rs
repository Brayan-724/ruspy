pub mod value;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use value::{AsBool, AsNumber, AsString, RuntimeValue, RuntimeVariable};

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
        let Some(var) = self.variables.borrow().get(&name).cloned() else {
            // Functions keep a secure context,
            // to manipulate external variables
            // use `global` keyword, that clones
            // variables to current scope
            let Some(parent) = self.parent.as_ref().filter(|_| !self.is_function) else {
                return self
                    .variables
                    .borrow_mut()
                    .entry(name)
                    .insert_entry(value.into())
                    .get()
                    .clone();
            };

            return parent.set_variable(name, value);
        };

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

    pub fn visit_conditional(
        self: &Rc<Self>,
        test: AstExpr,
        body: AstScope,
        otherwise: Option<AstScope>,
    ) {
        let test = self.visit_expr(test);

        if test.as_bool() {
            self.run(body);
        } else if let Some(otherwise) = otherwise {
            self.run(otherwise);
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
            } => RuntimeValue::Bool(!self.visit_expr(*right).as_bool()),
        }
    }

    pub fn visit_expr_binop(
        self: &Rc<Self>,
        op: AstBinaryOp,
        left: AstExpr,
        right: AstExpr,
    ) -> RuntimeValue {
        use AstBinaryOp::*;
        use RuntimeValue::*;

        match (op, self.visit_expr(left), self.visit_expr(right)) {
            ////// Number Primitives //////
            (Add, Number(a), Number(b)) => Number(a + b),
            (Div, Number(a), Number(b)) => Number(a / b),
            (Mul, Number(a), Number(b)) => Number(a * b),
            (Sub, Number(a), Number(b)) => Number(a - b),

            ////// Bool "Primitives" //////
            (Add, Bool(a), Bool(b)) => Number(a.as_num() + b.as_num()),
            (Add, Bool(false), Number(n)) | (Add, Number(n), Bool(false)) => Number(n),
            (Add, Bool(true), Number(n)) | (Add, Number(n), Bool(true)) => Number(n + 1),
            (Div, Bool(a), Bool(b)) => Number(a.as_num() / b.as_num()),
            (Div, Bool(a), Number(b)) => Number(a.as_num() / b),
            (Div, Number(a), Bool(b)) => Number(a / b.as_num()),
            (Mul, Bool(a), Number(b)) => Number(a.as_num() * b),
            (Mul, Bool(a), Bool(b)) => Number(a.as_num() * b.as_num()),
            (Mul, Number(a), Bool(b)) => Number(a * b.as_num()),
            (Sub, Bool(a), Bool(b)) => Number(a.as_num() - b.as_num()),
            (Sub, Bool(a), Number(b)) => Number(a.as_num() - b),
            (Sub, Number(a), Bool(b)) => Number(a - b.as_num()),

            ////// Concatenation //////
            (Add, String(a), Nil) => String(format!("{a}nil")),
            (Add, Nil, String(b)) => String(format!("nil{b}")),
            (Add, Bool(a), String(b)) => String(format!("{}{b}", a.as_string())),
            (Add, String(a), Bool(b)) => String(format!("{a}{}", b.as_string())),
            (Add, Number(a), String(b)) => String(format!("{a}{b}")),
            (Add, String(a), Number(b)) => String(format!("{a}{b}")),
            (Add, String(a), String(b)) => String(format!("{a}{b}")),

            ////// Multiplication //////
            (Mul, String(s), Bool(true)) | (Mul, Bool(true), String(s)) => String(s),
            (Mul, String(_), Bool(false)) | (Mul, Bool(false), String(_)) => String(String::new()),
            (Mul, Number(a), String(b)) => String(
                a.is_positive()
                    .then(|| b.repeat(a.unsigned_abs() as usize))
                    .unwrap_or_default(),
            ),
            (Mul, String(a), Number(b)) => String(
                b.is_positive()
                    .then(|| a.repeat(b.unsigned_abs() as usize))
                    .unwrap_or_default(),
            ),

            (_, _, String(_) | Nil) | (_, String(_) | Nil, _) => Nil,
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
