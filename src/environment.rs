use std::collections::HashMap;

use crate::object::Object;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new_with_outer(env: &Environment) -> Self {
        Self {
            store: HashMap::new(),
            outer: Some(Box::new(env.clone())),
        }
    }

    pub fn get(&self, k: &str) -> Option<&Object> {
        self.store
            .get(k)
            .or_else(|| self.outer.as_ref().and_then(|out| out.get(k)))
    }

    pub fn set(&mut self, k: String, v: Object) -> Option<Object> {
        self.store.insert(k, v)
    }
}
