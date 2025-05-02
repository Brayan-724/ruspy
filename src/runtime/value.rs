use std::cell::RefCell;
use std::rc::Rc;

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
