use std::collections::HashMap;

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

pub type Object = HashMap<String, Value>;
pub type Array = Vec<Value>;

#[derive(Clone, PartialEq)]
pub enum Value {
    Object(Object),
    Boolean(bool),
    Null,
    String(String),
    Number(f64),
    Array(Array),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Object(o) => write!(f, "{:?}", o),
            Value::Array(a) => write!(f, "{:?}", a),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Boolean(b) => write!(f, "Boolean({})", b),
            Value::Null => write!(f, "Null"),

            Value::String(s) => write!(f, "String({})", s),
            Value::Number(n) => write!(f, "Number({})", n),
            Value::Object(o) => write!(f, "{:?}", o),
            Value::Array(a) => write!(f, "{:?}", a),
        }
    }
}
