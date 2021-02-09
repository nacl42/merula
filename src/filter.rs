use crate::{Memo, Node, Key, Value};
use crate::memo::NodeType;

use std::convert::TryFrom;

// TODO: maybe get rid of Option<KeyFilter> and use KeyFilter::True
// as default. What would be the 'correct' default? Always True?

// TODO: get rid of with_... constructs
// use Default instead and init by calling { custom_filter ..Default::default()}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
    pub node_type: NodeType,
    pub key: Option<KeyFilter>,
    pub index: Option<IndexFilter>,
    pub value: Option<ValueFilter>
}

impl Default for NodeFilter {
    fn default() -> Self {
        NodeFilter {
            node_type: NodeType::Any,
            key: None,
            index: None,
            value: None
        }
    }
}

impl NodeFilter {

    pub fn with_node_type(mut self, node_type: NodeType) -> Self {
        self.node_type = node_type;
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
        // stepwise selection and filtering

        // (1) check for node type is done by selection of nodes
        let nodes = memo.node_iterator(self.node_type);
        
        nodes.filter(
            // (2) check for node key name
            |node| self.check_key(&node.key).unwrap_or(true)
        ).enumerate().filter(
            // (3) check for node index among selected keys
            |(n, _node)| self.check_index(*n).unwrap_or(true)
        ).filter(
            // (4) check for node value
            |(_n, node)| self.check_value(&node.value).unwrap_or(true)
        )
        // (5) return true if there is at least one match
            .next().is_some()
    }

    pub fn select<'a>(&'a self, memo: &'a Memo) -> impl Iterator<Item=&'a Node> {
        // stepwise selection and filtering

        // (1) check for node type is done by selection of nodes
        let nodes = memo.node_iterator(self.node_type);
        
        nodes.filter(
            // (2) check for node key name
            move |node| self.check_key(&node.key).unwrap_or(true)
        ).enumerate().filter(
            // (3) check for node index among selected keys
            move |(n, _node)| self.check_index(*n).unwrap_or(true)
        ).filter(
            // (4) check for node value
            move |(_n, node)| self.check_value(&node.value).unwrap_or(true)
        ).map(
            move |(_n, node)| node
        )
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

    pub fn check(&self, memo: &Memo) -> bool {
        self.node_filters.iter()
            .all(|nf: &NodeFilter| nf.check_memo(&memo))
    }
}


#[cfg(test)]
mod tests {
    use crate::filter::*;
    use crate::{Memo, Value};

    fn sample_memo() -> Memo {
        let mut memo = Memo::new("book", "The Lord of the Rings");
        memo.push(("author", "J. R. R. Tolkien"));
        memo.push(("character", "Bilbo Baggins"));
        memo.push(("character", "Samweis Gamdschie"));
        memo.push(("character", "Aragorn"));
        memo.push(("character", "Gandalf"));
        memo
    }

    #[test]
    fn test_key_filter() {
        let memo = sample_memo();

        let mut nf = NodeFilter::default();
        nf.key = Some(KeyFilter::Equals("author".into()));
        assert_eq!(nf.check_memo(&memo), true);

        let mut nf = NodeFilter::default();
        nf.key = Some(KeyFilter::Equals("character".into()));
        assert_eq!(nf.check_memo(&memo), true);

        let mut nf = NodeFilter::default();
        nf.key = Some(KeyFilter::Equals("tag".into()));
        assert_eq!(nf.check_memo(&memo), false);
    }

    #[test]
    fn test_select() {
        let memo = sample_memo();

        // NOTE: Testing is a little awkward, we might want to make up
        // a better and shorter notation
        let mut nf = NodeFilter::default();
        nf.key = Some(KeyFilter::Equals("author".into()));
        let mut nodes = nf.select(&memo);
        assert_eq!(nodes.next().unwrap().value, Value::Text("J. R. R. Tolkien".into()));
        assert_eq!(nodes.next().is_none(), true);

        let mut nf = NodeFilter::default();
        nf.key = Some(KeyFilter::Equals("character".into()));
        let mut nodes = nf.select(&memo);
        assert_eq!(nodes.next().unwrap().value, Value::Text("Bilbo Baggins".into()));
        assert_eq!(nodes.next().unwrap().value, Value::Text("Samweis Gamdschie".into()));
        assert_eq!(nodes.next().unwrap().value, Value::Text("Aragorn".into()));
        assert_eq!(nodes.next().unwrap().value, Value::Text("Gandalf".into()));
        assert_eq!(nodes.next().is_none(), true);
    }

    #[test]
    fn test_default_node_filter() {
        let nf = NodeFilter {
            node_type: NodeType::Header,
            ..Default::default()
        };
        assert_eq!(nf.node_type, NodeType::Header);
        assert_eq!(nf.key, None);
        assert_eq!(nf.value, None);
        assert_eq!(nf.index, None);
    }
}

