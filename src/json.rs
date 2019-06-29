use std::collections::HashMap;

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

pub type Object = HashMap<String, Value>;
pub type Array = Vec<Value>;

#[derive(Clone, PartialEq)]
pub enum Root {
    Object(Object),
    Array(Array),
}

#[derive(Clone, PartialEq)]
pub enum Value {
    Object(Object),
    Array(Array),
    Boolean(bool),
    Null,
    String(String),
    Number(f64),
}

impl Display for Root {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Root::Object(o) => write!(f, "{:?}", o),
            Root::Array(a) => write!(f, "{:?}", a),
        }
    }
}

impl fmt::Debug for Root {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Root::Object(o) => write!(f, "Object({:?})", o),
            Root::Array(a) => write!(f, "Array({:?})", a),
        }
    }
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

impl From<bool> for Value {
    fn from(b: bool) -> Value {
        Value::Boolean(b)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::String(s)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Value {
        Value::String(s.to_string())
    }
}

macro_rules! from_num_for_json {
    ( $( $t:ident )* ) => {
        $(
            impl From<$t> for Value {
                fn from(n: $t) -> Value {
                    Value::Number(n as f64)
                }
            }
        )*
    };
}

from_num_for_json!(i8 i16 i32 u8 u16 u32 u64 usize isize f32 f64);

#[allow(unused_macros)]
macro_rules! json {
    (null) => {
        Value::Null
    };
    ([ $( $element:tt ),* ]) =>{
        Value::Array(vec![ $( json!($element) ),*])
    };
    ({ $( $key:tt : $value:tt),* }) =>{
        Value::Object(vec![
            $( ($key.to_string(), json!($value)) ),*
        ].into_iter().collect())
    };
    ($other:tt) =>{
        Value::from($other)
    };
}

#[cfg(test)]
mod tests {
    use crate::json::Value;
    use std::collections::HashMap;

    macro_rules! hash {
        ( $( $t:expr),* ) => {
            {
                let mut temp_hash = HashMap::new();
                $(
                    temp_hash.insert($t.0, $t.1);
                )*
                temp_hash
            }
        };
    }

    #[test]
    fn json_null() {
        assert_eq!(json!(null), Value::Null);
    }
    #[test]
    fn json_array() {
        assert_eq!(json!([null]), Value::Array(vec![Value::Null]));
    }

    #[test]
    fn json_object() {
        assert_eq!(
            json!({ "null": null }),
            Value::Object(hash![("null".to_string(), Value::Null)])
        );
    }
    #[test]
    fn json_bool() {
        assert_eq!(json!(true), Value::Boolean(true));
    }
    #[test]
    fn json_number() {
        assert_eq!(json!(10e10), Value::Number(10e10));
    }
    #[test]
    fn json_string() {
        assert_eq!(json!("json"), Value::String("json".to_string()));

        let x = "x".to_string();
        assert_eq!(json!(x), Value::String("x".to_string()));
    }

}
