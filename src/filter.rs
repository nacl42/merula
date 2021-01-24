use crate::{Node, Key, Value};
use std::ops::BitAnd;
use std::convert::TryFrom;

type Predicate = dyn FnMut(&&Node) -> bool;

pub trait IntoPredicate {
    fn predicate(&self) -> Box<Predicate>;
}

#[derive(Debug)]
pub enum NodeFilter {
    True, // always true (internal use)
    HasKey(Key), // .key
    Contains(String), // ~ value
    XEquals(Value),
    Equals(String), // = value
    LessThan(f32), // < value
    LessOrEqual(f32), // <= value
    GreaterThan(f32), // > value
    GreaterOrEqual(f32), // >= value
    And(Box<NodeFilter>, Box<NodeFilter>)
}

impl From<(NodeFilter, NodeFilter)> for NodeFilter {
    fn from((c1, c2): (NodeFilter, NodeFilter)) -> NodeFilter {
        NodeFilter::And(Box::new(c1), Box::new(c2))
    }
}

impl IntoPredicate for NodeFilter {
    fn predicate(&self) -> Box<Predicate> {
        match self {
            NodeFilter::True => Box::new(move |node: &&Node| true),
            NodeFilter::HasKey(key) => {
                let key = key.clone();
                Box::new(move |node: &&Node| { node.key == key })
            },
            NodeFilter::Contains(value) => {
                let value = value.clone();
                Box::new(move |node: &&Node| {
                    match &node.value {
                        Value::Text(s) => s.contains(&value),
                        Value::MultiLineText(s, _) => s.contains(&value),
                        _ => false
                    }
                })
            },
            // TODO: experimental implementation of a comparison function
            // converts the node value to the type of `value`
            NodeFilter::XEquals(value) => {
                let value = value.clone();
                match value {
                    Value::Float(x) => {
                        Box::new(move |node: &&Node| {
                            if let Ok(other) = f32::try_from(&node.value) {
                                x == other
                            } else {
                                false
                            }
                        })
                    },
                    Value::Text(text) => {
                        Box::new(move |node: &&Node| {
                            text == node.value.to_string()                            
                        }) 
                    },
                    _ => Box::new(move |node: &&Node| false)
                }
            },
            NodeFilter::Equals(value) => {
                let value = value.clone();
                Box::new(move |node: &&Node| {
                    &node.value.to_string() == &value
                })
            },
            NodeFilter::LessThan(value) => {
                let value = value.clone();
                Box::new(move |node: &&Node| {
                    //DEBUG println!("Testing node {} => {:#?}",
                    //&node.value, f32::try_from(&node.value));
                    match f32::try_from(&node.value) {
                        Ok(x) => x < value,
                        Err(_) => false
                    }
                })
            },
            NodeFilter::LessOrEqual(value) => {
                let value = value.clone();
                Box::new(move |node: &&Node| {
                    //DEBUG println!("Testing node {} => {:#?}",
                    //&node.value, f32::try_from(&node.value));
                    match f32::try_from(&node.value) {
                        Ok(x) => x <= value,
                        Err(_) => false
                    }
                })
            },
            NodeFilter::GreaterThan(value) => {
                let value = value.clone();
                Box::new(move |node: &&Node| {
                    //DEBUG println!("Testing node {} => {:#?}",
                    //&node.value, f32::try_from(&node.value));
                    match f32::try_from(&node.value) {
                        Ok(x) => x > value,
                        Err(_) => false
                    }
                })
            },
            NodeFilter::GreaterOrEqual(value) => {
                let value = value.clone();
                Box::new(move |node: &&Node| {
                    //DEBUG println!("Testing node {} => {:#?}",
                    //&node.value, f32::try_from(&node.value));
                    match f32::try_from(&node.value) {
                        Ok(x) => x >= value,
                        Err(_) => false
                    }
                })
            },
            NodeFilter::And(c1, c2) => {
                let mut p1 = c1.predicate();
                let mut p2 = c2.predicate();
                Box::new(move |node: &&Node| { p1(node) && p2(node) })
            }
        }
    }
}


impl BitAnd for NodeFilter {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        NodeFilter::And(Box::new(self), Box::new(rhs))
    }
}
