//! A Memo is a list of Nodes.
//!
//! Please note that this might not be the perfectly appropriate term,
//! when considering graph theory.
//!
//! The `group` and `title` are stored in the first Node, which is
//! called the header node. A memo can belong to only one group and
//! has exactly one title.
//!
//! Additional information can be stored in the subsequent Nodes,
//! which are called data nodes.
//!
//! An Node consists of a Key and a Value, with optional attributes.
//! Attributes are stored as a HashMap of Key/Value pairs.  Both
//! the header node and the data nodes can have attributes.
//!

use crate::value::{Key, Value};
use crate::node::Node;


/// A Memo consists of multiple nodes, the first one being the header
/// node and the subsequent ones being the data nodes.
#[derive(Debug)]
pub struct Memo {
    nodes: Vec<Node>
}

impl Memo {
    pub fn new<K, V>(group: K, title: V) -> Self
        where K: Into<Key>, V: Into<Value>
    {
        Memo {
            nodes: vec![Node::new(group, title)]
        }
    }

    pub fn push(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn group(&self) -> Key {
        self.nodes[0].key.clone()
    }

    pub fn title(&self) -> Value {
        self.nodes[0].value.clone()
    }

    pub fn header(&self) -> &Node {
        &self.nodes[0]
    }
    
    pub fn data(&self) -> impl Iterator<Item=&Node> {
        self.nodes[1..].iter()
    }

    pub fn data_count(&self) -> usize {
        self.nodes.len() - 1
    }
    
    pub fn last(&self) -> &Node {
        &self.nodes[self.nodes.len() - 1]
    }

    pub fn last_mut(&mut self) -> &mut Node {
        let index = self.nodes.len() - 1;
        self.nodes.get_mut(index).unwrap()
    }

    /// Return reference to the first Node that matches the given key.
    pub fn get<K: Into<Key>>(&self, key: K) -> Option<&Node> {
        let key = key.into();
        self.data()
            .find(|n| n.key == key)
    }

    /// Return vector to references to all nodes matching the given
    /// key.
    pub fn get_vec<K: Into<Key>>(&self, key: K) -> Vec<&Node> {
        let key = key.into();
        self.data()
            .filter(|n| n.key == key)
            .collect::<Vec<&Node>>()
    }    
}


impl std::fmt::Display for Memo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prefix = "@";
        for node in &self.nodes {
            writeln!(f, "{}{} {}", prefix, node.key, node.value)?;
            for (key, value) in node.attrs.iter() {
                writeln!(f, "+{} {}", key, value)?;
            }
            // TODO: implement text output
            //if let Some(text) = &node.text {
            //    writeln!(f, "{}", text);
            //}
            prefix = ".";
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn sample_memo() -> Memo {
        let mut memo = Memo::new("book", "The Lord of the Rings");
        memo.push(Node::new("author", "J.R.R. Tolkien"));
        memo.push(Node::new("character", "Bilbo Baggins"));
        memo.push(Node::new("character", "Samweis Gamdschie"));
        memo
    }
    
    #[test]
    fn check_header() {
        let memo = sample_memo();
        assert_eq!(memo.group(), "book");
        assert_eq!(memo.title(), Value::from("The Lord of the Rings"));
    }

    #[test]
    fn check_data() {
        let memo = sample_memo();
        let node = memo.data().next().unwrap();
        assert_eq!(node.key, "author");
        assert_eq!(node.value, Value::from("J.R.R. Tolkien"));
    }

    #[test]
    fn data_count() {
        let memo = sample_memo();
        assert_eq!(memo.data_count(), 3);
    }
}
