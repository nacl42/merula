//! A Value is an enum for the different types of values that a
//! Node can hold.
//!
//! Possible variants are:
//! - `Value::Text` holding a single-line string (no newlines)
//! - `Value::MultiLineText` holding a multi-line string (with newlines)
//! - `Value::Integer` holding a 32-bit integer number
//! - `Value::Float` holding a 32-bit floating point number
//! - `Value::Bool` holding a true/false value
//!
//! A Value can be constructed using the From Trait:
//! - String or &str will be converted into `Value::Text` or to
//!   `Value::MultiLineText` if it contains at least one newline.
//! - i32 will be converted into `Value::Integer`
//! - f32 will be converted into `Value::Float`
//! - bool will be converted into `Value::Bool`
//!

// TODO: Value::Date, Value::DateTime, Value::Time
// TODO: Value::Ref(group, title), Value::RefById(id)

use std::convert::TryFrom;

pub type Key = String;

/// A Value is an enum for the different types of values that a
/// Node can hold.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Text(String),
    MultiLineText(String, String), // (text, sep)
    Integer(i32),
    Float(f32),
    Bool(bool)
}

impl Value {
    /// Returns true if Value is of type Value::Bool.
    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false
        }
    }

    /// Returns true if Value is some kind of number (Value::Integer or Value::Float).
    pub fn is_number(&self) -> bool {
        match self {
            Value::Integer(_) | Value::Float(_) => true,
            _ => false
        }
    }

    /// Returns true if Value is a Value::Text.
    pub fn is_text(&self) -> bool {
        match self {
            Value::Text(_) | Value::MultiLineText(_, _) => true,
            _ => false
        }
    }

    /// Returns true if Value is an Value::Integer
    pub fn is_integer(&self) -> bool {
        match self {
            Value::Integer(_) => true,
            _ => false
        }
    }

    /// Returns true if Value is a Value::Float
    pub fn is_float(&self) -> bool {
        match self {
            Value::Float(_) => true,
            _ => false
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Text(text) => write!(f, "{}", text),
            Value::MultiLineText(text, sep) =>
                write!(f, "<<{}\n{}\n{}", sep, text, sep),
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(x) => write!(f, "{}", x),
            Value::Bool(b) => write!(f, "{}", b)
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Value {
        if s.contains("\n") {
            Value::MultiLineText(s.to_string(), "EOF".to_string())
        } else {
            Value::Text(s.to_string())
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Value {
        // TODO: check for newlines
        if s.contains("\n") {
            Value::MultiLineText(s, "EOF".to_string())
        } else {
            Value::Text(s)
        }
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

impl <'a> TryFrom<&'a Value> for f32 {
    type Error = &'static str;
    
    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(x) => Ok(x.clone()),
            Value::Integer(x) => Ok(x.clone() as f32),
            Value::Text(text) => {
                match text.parse::<f32>() {
                    Ok(x) => Ok(x),
                    _ => Err("Value::Text cannot be parsed as float")
                }
            },
            _ => Err("Value is neither Value::Float nor Value::Integer")
        }
    }
}
