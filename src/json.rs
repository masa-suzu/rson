use std::collections::HashMap;

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

pub type Object = HashMap<String, Value>;

#[derive(Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Object(Object),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Object(o) => write!(f, "{:?}", o),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "String({})", s),
            Value::Number(n) => write!(f, "Number({})", n),
            Value::Object(o) => write!(f, "{:?}", o),
        }
    }
}
