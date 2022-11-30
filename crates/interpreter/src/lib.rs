use std::{collections::HashMap, rc::Rc, cell::RefCell};

pub enum Value {
    String(String),
}

pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    values: HashMap<String, Value>,
}
