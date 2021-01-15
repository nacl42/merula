use crate::{Node, Key, Value};

type Predicate = dyn FnMut(&&Node) -> bool;

pub trait NodeFilter {
    fn predicate(&self) -> Box<Predicate>;
}


pub struct KeyFilter {
    key: Key,
    // enum Operator        
}

pub enum Comparison {
    HasKey(Key), // .
    Contains(String), // ~
    //GreaterThan(Key, Value), // >
    //LessThan(Key, Value), // <
    //Equal(Key, Value), // =
}

impl NodeFilter for Comparison {
    fn predicate(&self) -> Box<Predicate> {
        match self {
            Comparison::HasKey(key) => {
                let key = key.clone();
                Box::new(move |node: &&Node| { node.key == key })
            },
            Comparison::Contains(value) => {
                let value = value.clone();
                Box::new(move |node: &&Node| {
                    match &node.value {
                        Value::Text(s) => s.contains(&value),
                        Value::MultiLineText(s, _) => s.contains(&value),
                        _ => false
                    }
                })
            }
        }
    }
}


impl KeyFilter {
    pub fn new<K>(key: K) -> Self
    where K: Into<Key>
    {
        KeyFilter {
            key: key.into()
        }
    }
}

impl NodeFilter for KeyFilter {
    fn predicate(&self) -> Box<Predicate> {
        let key = self.key.clone();
        Box::new(move |node: &&Node| { node.key == key })
    }
}
