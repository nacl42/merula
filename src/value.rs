
pub type Key = String;

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
