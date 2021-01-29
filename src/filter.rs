use crate::{Memo, Node, Key, Value};
use std::convert::TryFrom;

#[derive(Debug)]
pub enum KeyFilter {
    True,
    Equals(String)
}

impl KeyFilter {
    pub fn check(&self, key: &Key) -> bool {
        match self {
            KeyFilter::True => true,
            KeyFilter::Equals(x) => key == x,
        }
    }
}

#[derive(Debug)]
pub enum ValueFilter {
    True,
    Equals(String),
    Contains(String),
    LessThan(f32),
    MoreThan(f32),
    AtLeast(f32),
    AtMost(f32)
}


impl ValueFilter {
    pub fn check(&self, value: &Value) -> bool {
        match self {
            ValueFilter::True => true,
            ValueFilter::Equals(x) => &value.to_string() == x,
            ValueFilter::Contains(x) => {
                match &value {
                    Value::Text(value_text) => value_text.contains (x),
                    Value::MultiLineText(value_text, _) => value_text.contains(x),
                    _ => false
                }
            },
            ValueFilter::LessThan(x) => {
                match f32::try_from(value) {
                    Ok(value_f32) => value_f32 < *x,
                    Err(_) => false
                }
            },
            ValueFilter::MoreThan(x) => {
                match f32::try_from(value) {
                    Ok(value_f32) => value_f32 > *x,
                    Err(_) => false
                }                
            },
            ValueFilter::AtMost(x) => {
                match f32::try_from(value) {
                    Ok(value_f32) => value_f32 <= *x,
                    Err(_) => false
                }
            },
            ValueFilter::AtLeast(x) => {
                match f32::try_from(value) {
                    Ok(value_f32) => value_f32 >= *x,
                    Err(_) => false
                }                
            }

        }
    }
}

#[derive(Debug)]
pub struct NodeFilter {
    pub key: Option<KeyFilter>,
    pub value: Option<ValueFilter>
}

impl NodeFilter {
    pub fn new() -> Self {
        NodeFilter {
            key: None,
            value: None
        }
    }
    
    pub fn check_node(&self, node: &Node) -> Option<bool> {
        match (&self.key, &self.value){
            (None, None) => None,
            (Some(key), None) => Some(key.check(&node.key)),
            (Some(key), Some(value)) => Some(key.check(&node.key) && value.check(&node.value)),
            (None, Some(value)) => Some(value.check(&node.value))
        }
    }
    
    pub fn check_key(&self, key: &Key) -> Option<bool> {
        match &self.key {
            Some(filter) => Some(filter.check(key)),
            _ => None
        }
    }

    pub fn check_value(&self, value: &Value) -> Option<bool> {
        match &self.value {
            Some(filter) => Some(filter.check(value)),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct MemoFilter {
    pub node_filters: Vec<NodeFilter>
}

impl MemoFilter {
    pub fn new() -> MemoFilter {
        MemoFilter {
            node_filters: vec!()
        }
    }

    pub fn and(mut self, nf: NodeFilter) -> Self {
        self.node_filters.push(nf);
        self
    }

    pub fn add_filter(&mut self, nf: NodeFilter) {
        self.node_filters.push(nf);
    }

    pub fn check_memo(&self, memo: &Memo) -> bool {
        self.node_filters.iter().all(
            |nf| memo.nodes().any(|node| nf.check_node(&node).unwrap_or(false))
        )
    }
}
