use std::collections::HashMap;

pub struct DB {
    store: HashMap<String, String>,
}

impl DB {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.store.remove(key)
    }

    pub fn update(&mut self, key: &str, value: String) -> bool {
        if let Some(v) = self.store.get_mut(key) {
            *v = value;
            true
        } else {
            false
        }
    }
}