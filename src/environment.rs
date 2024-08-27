
use std::collections::HashMap;

use crate::object::Object;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn get(&self, k: &str) -> Option<&Object> {
        self.store.get(k)
    }
    
    pub fn set(&mut self, k: String, v: Object) -> Option<Object> {
        self.store.insert(k, v)
    }
}

