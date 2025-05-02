use std::cell::RefCell;
use std::rc::Rc;

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

impl Into<RuntimeVariable> for RuntimeValue {
    fn into(self) -> RuntimeVariable {
        self.wrap()
    }
}

impl RuntimeValue {
    pub fn wrap(self) -> RuntimeVariable {
        RuntimeVariable(Rc::new(RefCell::new(self)))
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
