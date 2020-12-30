//! A Node represents a single piece of information, a Key-Value pair.
//!
//! A Node consists of key, a value and a map of attributes.
//!

use std::collections::HashMap;
use crate::value::{Key, Value};

/// A Node consists of key, a value and a map of attributes.
#[derive(Debug)]
pub struct Node {
    pub key: Key,
    pub value: Value,
    pub attrs: HashMap<Key, Value>
}

impl Node {
    pub fn new<K, V>(key: K, value: V) -> Self
    where K: Into<Key>, V: Into<Value>
    {
        Node {
            key: key.into(),
            value: value.into(),
            attrs: HashMap::new()
        }
    }
}
