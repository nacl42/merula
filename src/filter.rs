use crate::{Node, Key};

type Predicate = dyn FnMut(&&Node) -> bool;

pub trait NodeFilter {
    fn predicate(&self) -> Box<Predicate>;
}


pub struct KeyFilter {
    name: Key
}

impl KeyFilter {
    pub fn new<K>(name: K) -> Self
    where K: Into<Key>
    {
        KeyFilter {
            name: name.into()
        }
    }
}

impl NodeFilter for KeyFilter {
    fn predicate(&self) -> Box<Predicate> {
        let name = self.name.clone();
        Box::new(move |node: &&Node| { node.key == name })
    }
}
