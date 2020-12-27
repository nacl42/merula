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
    pub items: Vec<Item>
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
