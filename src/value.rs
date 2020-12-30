//! A Value is an enum for the different types of values that a
//! Node can hold.
//!
//! Possible variants are:
//! - `Value::Text` holding a single-line string (no newlines)
//! - `Value::Integer` holding a 32-bit integer number
//! - `Value::Float` holding a 32-bit floating point number
//! - `Value::Bool` holding a true/false value
//!
//! A Value can be constructed using the From Trait:
//! - String or &str will be converted into `Value::Text`
//! - i32 will be converted into `Value::Integer`
//! - f32 will be converted into `Value::Float`
//! - bool will be converted into `Value::Bool`
//!

pub type Key = String;

/// A Value is an enum for the different types of values that a
/// Node can hold.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Text(String),
    Integer(i32),
    Float(f32),
    Bool(bool)
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Text(text) => write!(f, "{}", text),
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(x) => write!(f, "{}", x),
            Value::Bool(b) => write!(f, "{}", b)
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Value {
        // TODO: check for newlines
        Value::Text(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Value {
        // TODO: check for newlines
        Value::Text(s)
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Value {
        Value::Integer(n)
    }
}

impl From<f32> for Value {
    fn from(x: f32) -> Value {
        Value::Float(x)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Value {
        Value::Bool(b)
    }
}
