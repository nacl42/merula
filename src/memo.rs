//! A Memo is a list of Items.
//!
//! The `group` and `title` are stored in the first Item, which is
//! called the header item.
//!
//! Additional information can be stored in the subsequent Items,
//! which are called data items.
//!
//! An Item consists of a Key and a Value, with optional attributes.
//! Attributes are stored as a HashMap of Key/Value pairs.
//!

use std::collections::HashMap;
use crate::value::{Key, Value};

#[derive(Debug)]
pub struct Item {
    pub key: Key,
    pub value: Value,
    pub attrs: HashMap<Key, Value>
}

impl Item {
    pub fn new<K, V>(key: K, value: V) -> Self
    where K: Into<Key>, V: Into<Value>
    {
        Item {
            key: key.into(),
            value: value.into(),
            attrs: HashMap::new()
        }
    }
}

#[derive(Debug)]
pub struct Memo {
    items: Vec<Item>
}

impl Memo {
    pub fn new<K, V>(group: K, title: V) -> Self
        where K: Into<Key>, V: Into<Value>
    {
        Memo {
            items: vec![Item::new(group, title)]
        }
    }

    pub fn push(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn group(&self) -> Key {
        self.items[0].key.clone()
    }

    pub fn title(&self) -> Value {
        self.items[0].value.clone()
    }

    pub fn header(&self) -> &Item {
        &self.items[0]
    }
    
    pub fn data(&self) -> impl Iterator<Item=&Item> {
        self.items[1..].iter()
    }

    pub fn data_count(&self) -> usize {
        self.items.len() - 1
    }
    
    pub fn last(&self) -> &Item {
        &self.items[self.items.len() - 1]
    }

    pub fn last_mut(&mut self) -> &mut Item {
        let index = self.items.len() - 1;
        self.items.get_mut(index).unwrap()
    }
    
    pub fn get<K: Into<Key>>(&self, key: K) -> Option<&Item> {
        let key = key.into();
        self.data()
            .find(|n| n.key == key)
    }

    pub fn get_vec<K: Into<Key>>(&self, key: K) -> Vec<&Item> {
        let key = key.into();
        self.data()
            .filter(|n| n.key == key)
            .collect::<Vec<&Item>>()
    }    
}


impl std::fmt::Display for Memo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prefix = "@";
        for item in &self.items {
            writeln!(f, "{}{} {}", prefix, item.key, item.value)?;
            for (key, value) in item.attrs.iter() {
                writeln!(f, "+{} {}", key, value)?;
            }
            // TODO: implement text output
            //if let Some(text) = &item.text {
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
        memo.push(Item::new("author", "J.R.R. Tolkien"));
        memo.push(Item::new("character", "Bilbo Baggins"));
        memo.push(Item::new("character", "Samweis Gamdschie"));
        memo
    }
    
    #[test]
    fn check_header() {
        let memo = sample_memo();
        let header = memo.header();
        assert_eq!(header.key, "book");
        assert_eq!(header.value, Value::from("The Lord of the Rings"));
    }

    #[test]
    fn check_data() {
        let memo = sample_memo();
        let item = memo.data().next().unwrap();
        assert_eq!(item.key, "author");
        assert_eq!(item.value, Value::from("J.R.R. Tolkien"));
    }

    #[test]
    fn data_count() {
        let memo = sample_memo();
        assert_eq!(memo.data_count(), 3);
    }
}
