use crate::{Memo, Node, Key, Value};
use std::convert::TryFrom;

// TODO: implement step-by-step filter for header and data nodes
// TODO: maybe try to return std::iter::Slice<'_, &Node> in memo.data()

#[derive(Debug)]
pub enum PrefixFilter {
    Any,
    Header,
    Data
}

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
pub enum IndexFilter {
    Any,
    Single(usize),
    Range(usize, usize), // from, to
}

impl IndexFilter {
    pub fn check(&self, index: usize) -> bool {
        match self {
            IndexFilter::Single(n) => n == &index,
            IndexFilter::Range(from, to) => (from >= &index) && (&index <= to),
            _ => false
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
    pub prefix: Option<PrefixFilter>,
    pub key: Option<KeyFilter>,
    pub index: Option<IndexFilter>,
    pub value: Option<ValueFilter>
}

impl NodeFilter {
    pub fn new() -> Self {
        NodeFilter {
            prefix: None,
            key: None,
            index: None,
            value: None
        }
    }

    pub fn with_prefix(mut self, prefix: PrefixFilter) -> Self {
        self.prefix = Some(prefix);
        self
    }
    
    pub fn with_key(mut self, key: KeyFilter) -> Self {
        self.key = Some(key);
        self            
    }

    pub fn with_index(mut self, index: IndexFilter) -> Self {
        self.index = Some(index);
        self
    }

    pub fn with_value(mut self, value: ValueFilter) -> Self {
        self.value = Some(value);
        self
    }

    pub fn check_index(&self, index: usize) -> Option<bool> {
        match &self.index {
            Some(filter) => Some(filter.check(index)),
            _ => None
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

    pub fn check_node_n(&self, node: &Node, n: usize) -> Option<bool> {
        match self.prefix {
            Some(PrefixFilter::Header) if n > 0 => return Some(false),
            Some(PrefixFilter::Data) if n == 0 => return Some(false),
            _ => {}
        }
        self.check_node(node)    
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

    pub fn check_memo(&self, memo: &Memo) -> bool {
        // TODO: check for index if available
        // for header, only index 0 makes sense
        // for data and all nodes, we have to filter by keys first
        // then enumerate over all matches and apply the index filter
        // finally, we apply the value filter
        match self.prefix {
            Some(PrefixFilter::Data) => {
                memo.data().filter(|node| self.check_node(node).unwrap_or(false))
                    .next().is_some()

            },
            Some(PrefixFilter::Header) => {
                self.check_node(memo.header()).unwrap_or(false)
            },
            _ => {
                memo.nodes().filter(
                    |node| self.check_key(&node.key).unwrap_or(false)
                ).enumerate().filter(
                    |(n, _node)| self.check_index(*n).unwrap_or(true)
                ).filter(
                    |(_n, node)| self.check_value(&node.value).unwrap_or(true)
                ).next().is_some()
            }
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

    pub fn add(&mut self, nf: NodeFilter) {
        self.node_filters.push(nf);
    }

    pub fn extend(&mut self, mf: MemoFilter){
        for nf in mf.node_filters {
            self.node_filters.push(nf)
        }
    }

    pub fn check_memo(&self, memo: &Memo) -> bool {
        self.node_filters.iter()
            .all(|nf: &NodeFilter| nf.check_memo(&memo))
    }
}
