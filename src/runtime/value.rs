use std::cell::RefCell;
use std::rc::Rc;

pub trait AsBool {
    fn as_bool(&self) -> bool;
}

pub trait AsNumber {
    fn as_num(&self) -> i64;
}

pub trait AsString {
    fn as_string(&self) -> &'static str;
}

#[derive(Debug, Clone)]
pub struct RuntimeVariable(pub Rc<RefCell<RuntimeValue>>);

#[derive(Default, Debug, Clone)]
pub enum RuntimeValue {
    #[default]
    Nil,
    Bool(bool),
    Number(i64),
    String(String),
}

impl From<RuntimeValue> for RuntimeVariable {
    fn from(val: RuntimeValue) -> Self {
        val.wrap()
    }
}

impl RuntimeValue {
    pub fn wrap(self) -> RuntimeVariable {
        RuntimeVariable(Rc::new(RefCell::new(self)))
    }
}

impl AsBool for RuntimeValue {
    fn as_bool(&self) -> bool {
        match self {
            RuntimeValue::Nil => false,
            RuntimeValue::Bool(b) => *b,
            RuntimeValue::Number(n) => *n != 0,
            RuntimeValue::String(s) => !s.is_empty(),
        }
    }
}

impl AsNumber for bool {
    fn as_num(&self) -> i64 {
        match self {
            true => 1,
            false => 0,
        }
    }
}

impl AsString for bool {
    fn as_string(&self) -> &'static str {
        match self {
            true => "True",
            false => "False",
        }
    }
}
