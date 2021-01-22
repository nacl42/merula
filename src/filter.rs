use crate::{Node, Key, Value};
use std::ops::BitAnd;

type Predicate = dyn FnMut(&&Node) -> bool;

pub trait IntoPredicate {
    fn predicate(&self) -> Box<Predicate>;
}

#[derive(Debug)]
pub enum NodeFilter {
    HasKey(Key), // .key
    ContainsValue(String), // ~value
    EqualsValue(String), // =value
    And(Box<NodeFilter>, Box<NodeFilter>)
        
    //GreaterThan(Key, Value), // >
    //LessThan(Key, Value), // <
    //Equal(Key, Value), // =
}

impl From<(NodeFilter, NodeFilter)> for NodeFilter {
    fn from((c1, c2): (NodeFilter, NodeFilter)) -> NodeFilter {
        NodeFilter::And(Box::new(c1), Box::new(c2))
    }
}

impl IntoPredicate for NodeFilter {
    fn predicate(&self) -> Box<Predicate> {
        match self {
            NodeFilter::HasKey(key) => {
                let key = key.clone();
                Box::new(move |node: &&Node| { node.key == key })
            },
            NodeFilter::ContainsValue(value) => {
                let value = value.clone();
                Box::new(move |node: &&Node| {
                    match &node.value {
                        Value::Text(s) => s.contains(&value),
                        Value::MultiLineText(s, _) => s.contains(&value),
                        _ => false
                    }
                })
            },
            NodeFilter::EqualsValue(value) => {
                let value = value.clone();
                Box::new(move |node: &&Node| {
                    &node.value.to_string() == &value
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
