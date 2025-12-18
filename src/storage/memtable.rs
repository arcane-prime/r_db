use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::env;
use serde_json::Value;
use std::mem;


pub type DataMap = BTreeMap<String, Value>;

pub struct MemTable { 
    data: DataMap,
    threshold: usize,
}

impl MemTable { 
    pub fn new() -> Arc<Mutex<Self>> {
        let threshold = env::var("MEMORY_THRESHOLD")
            .unwrap_or_else(|_| {
                eprintln!("WARNING: MEMORY_THRESHOLD not set. Using default of 10.");
                "10".to_string()
            })
            .parse::<usize>()
            .unwrap_or_else(|_| {
                eprintln!("ERROR: MEMORY_THRESHOLD is not a valid number. Using default of 1000.");
                1000
            });

        println!("MemTable initialized with capacity: {} entries.", threshold);

        Arc::new(Mutex::new(MemTable {
            data: BTreeMap::new(),
            threshold,
        }))
    }

    pub fn put(&mut self, key: String, value: Value) -> bool { 
        self.data.insert(key, value);

        self.data.len() >= self.threshold
    }

    pub fn get(&self, key: &str) -> Option<Value> { 
        self.data.get(key).cloned()
    }

    pub fn take_data(&mut self) -> DataMap {
        let old_data = mem::take(&mut self.data);

        old_data
    } 
}