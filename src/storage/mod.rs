pub mod memtable;
pub use memtable::{MemTable};

use std::sync::{Arc, Mutex};
use std::io::{Result, Error, ErrorKind};
use std::fs::{File, create_dir_all};
use serde_json::Value;


pub struct StorageManager { 
    memtable: Arc<Mutex<MemTable>>,
}

impl StorageManager { 
    pub fn new () -> Result<Self> { 
        let memtable = MemTable::new();

        Ok(StorageManager{
            memtable: memtable
        })
    }

    pub fn put (&self, key: String, value: Value) -> Result<()> { 
        let should_flush;
        { 
            let mut memtable = self.memtable.lock()
            .map_err(|e| Error::new(ErrorKind::Other, format!("MemTable lock poisoned: {}", e)))?;

             should_flush = memtable.put(key, value);
        }

        if should_flush { 
            println!("\n>>> MEMORY THRESHOLD REACHED! Triggering flush to disk...");
            self.flush_data()?;
        }

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        let memtable = self.memtable.lock().ok()?;

        if let Some(value_ref) = memtable.get(key) {
            return Some(value_ref.clone()); 
        }
        
        None
    }

    pub fn flush_data(&self) -> Result<()> { 
        let mut memtable = self.memtable.lock()
            .map_err(|e| Error::new(ErrorKind::Other, format!("MemTable lock poisoned during flush: {}", e)))?;

        let data_to_flush = memtable.take_data();
        drop(memtable);

        create_dir_all("data")
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create data dir: {}", e)))?;

        let timestamp = chrono::Utc::now().timestamp_nanos_opt().unwrap();
        let filename = format!("data/sstable_{}.dat", timestamp);

        println!("Writing {} records to file: {}", data_to_flush.len(), filename);

        let file = File::create(&filename)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create file {}: {}", filename, e)))?;

        serde_json::to_writer_pretty(&file, &data_to_flush)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to write data to {}: {}", filename, e)))?;

        println!("Flush successful! File {} created.", filename);

        Ok(())
    }
}