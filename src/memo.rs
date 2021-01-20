//! A Memo is a list of Nodes.
//!
//! Please note that this might not be the perfectly appropriate term,
//! when considering graph theory.
//!
//! The `collection` and `title` are stored in the first Node, which
//! is called the header node. A memo can belong to only one
//! collection and has exactly one title.
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

use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;

pub type MemoId = u64;

/// A Memo consists of multiple nodes, the first one being the header
/// node and the subsequent ones being the data nodes.
#[derive(Debug)]
pub struct Memo {
    nodes: Vec<Node>,
    attrs: HashMap<Key, Value>
}

impl Memo {
    /// Constructs a new, empty Memo.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut memo = Memo::new("book", "The Lord of the Rings");
    /// ```
    pub fn new<K, V>(collection: K, title: V) -> Self
        where K: Into<Key>, V: Into<Value>
    {
        Memo {
            nodes: vec![Node::new(collection, title)],
            attrs: HashMap::new()
        }
    }

    /// Adds given Node `node` to the Memo.
    pub fn push<N>(&mut self, node: N)
    where N: Into<Node> {
        self.nodes.push(node.into());
    }

    /// Adds given Node `node` to the Memo and returns the instance of
    /// the Memo.  With this method, the builder pattern can be used
    /// to add multiple nodes.
    pub fn with<N>(mut self, node: N) -> Self
    where N: Into<Node> {
        self.nodes.push(node.into());
        self
    }
    
    /// Returns Memo collection (key of header node).
    pub fn collection(&self) -> Key {
        self.header().key.clone()
    }

    /// Returns Memo title as string (value of header node).
    pub fn title(&self) -> String {
        self.header().value.to_string()
    }

    /// Returns reference to header node.
    pub fn header(&self) -> &Node {
        &self.nodes[0]
    }

    /// Returns unique header id
    pub fn id(&self) -> MemoId {
        let mut s = DefaultHasher::new();
        self.collection().hash(&mut s);
        self.title().hash(&mut s);
        s.finish()
    }

    /// Returns iterator to all nodes (header and data)
    pub fn nodes(&self) -> impl Iterator<Item=&Node> {
        self.nodes.iter()
    }
    
    /// Returns iterator to data nodes.
    pub fn data(&self) -> impl Iterator<Item=&Node> {
        self.nodes[1..].iter()
    }

    /// Returns number of data nodes.
    pub fn data_count(&self) -> usize {
        self.nodes.len() - 1
    }

    /// Returns reference to last inserted data node.
    pub fn last(&self) -> &Node {
        &self.nodes[self.nodes.len() - 1]
    }

    /// Returns mutable reference to last inserted data node.
    pub fn last_mut(&mut self) -> &mut Node {
        let index = self.nodes.len() - 1;
        self.nodes.get_mut(index).unwrap()
    }

    /// Returns reference to the first data node that matches the given key.
    pub fn get<K: Into<Key>>(&self, key: K) -> Option<&Node> {
        let key = key.into();
        self.nodes.iter().find(|n| n.key == key)
    }

    /// Returns vector to references to all data nodes matching the
    /// given key.
    pub fn get_vec<K: Into<Key>>(&self, key: K) -> Vec<&Node> {
        let key = key.into();
        self.nodes.iter()
            .filter(|n| n.key == key)
            .collect::<Vec<&Node>>()
    }

    /// Returns true if the Memo contains at least one data node with
    /// the given key.
    pub fn contains_key<K: Into<Key>>(&self, key: K) -> bool {
        let key = key.into();
        match self.nodes.iter().find(|&node| node.key == key) {
            Some(_) => true,
            _ => false
        }            
    }

    /// Returns true if the Memo has no data nodes.
    pub fn is_empty(&self) -> bool {
        self.nodes.len() < 2
    }
}


impl std::fmt::Display for Memo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // print header
        let prefix = "@";
        let header = self.header();
        writeln!(f, "{}{} {}", prefix, header.key, header.value)?;
        for (key, value) in header.attrs.iter() {
            writeln!(f, "+{} {}", key, value)?;
        }
        
        // print data nodes
        let prefix = ".";
        for node in &self.nodes {
            writeln!(f, "{}{} {}", prefix, node.key, node.value)?;
            for (key, value) in node.attrs.iter() {
                writeln!(f, "+{} {}", key, value)?;
            }
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
        assert_eq!(memo.collection(), "book");
        assert_eq!(memo.title(), "The Lord of the Rings");
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

    #[test]
    fn builder_pattern() {
        let memo1 = Memo::new("book", "The Lord of the Rings")
            .with(Node::new("author", "J.R.R. Tolkien"))
            .with(Node::new("character", "Samweis Gamdschie"));

        let mut memo2 = Memo::new("book", "The Lord of the Rings");
        memo2.push(Node::new("author", "J.R.R. Tolkien"));
        memo2.push(Node::new("character", "Samweis Gamdschie"));

        assert_eq!(memo1.title(), memo2.title());
        assert_eq!(memo1.collection(), memo2.collection());

        for (node1, node2) in memo1.data().zip(memo2.data()) {
            assert_eq!(node1.key, node2.key);
            assert_eq!(node1.value, node2.value);
        }
    }
}

