pub mod memtable;
pub use memtable::MemTable;

use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::{File, create_dir_all};
use std::io::{Error, ErrorKind, Result};
use std::sync::{Arc, Mutex};

type Range = (u64, u64);

pub struct StorageManager {
    memtable: Arc<Mutex<MemTable>>,
    range_map: Vec<Range>,
}

impl StorageManager {
    pub fn new() -> Result<Self> {
        let memtable = MemTable::new();
        let range_map = Vec::new();

        Ok(StorageManager {
            memtable: memtable,
            range_map,
        })
    }

    pub fn put(&mut self, value: Value) -> Result<String> {
        let (should_flush, key) = {
            let mut memtable = self.memtable.lock().map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("MemTable lock poisoned during put: {}", e),
                )
            })?;

            let next_num = if memtable.is_empty() {
                match self.range_map.last() {
                    Some((_, last_max)) => last_max + 1,
                    None => 1,
                }
            } else {
                memtable
                    .max_key_as_u64()
                    .unwrap_or(0)
                    .checked_add(1)
                    .ok_or_else(|| Error::new(ErrorKind::Other, "id overflow"))?
            };

            let key = format!("key_{}", next_num);
            let should_flush = memtable.put(key.clone(), value);
            (should_flush, key)
        };

        if should_flush {
            println!("\n>>> MEMORY THRESHOLD REACHED! Triggering flush to disk...");
            self.flush_data_v2()?;
        }

        Ok(key)
    }

    pub fn get(&self, key: &str) -> Option<Value> { 
        let sstable_filename = self.get_sstable_filename(key)?;
        let file = File::open(sstable_filename).ok()?;
        let data: BTreeMap<String, Value> = serde_json::from_reader(file).ok()?;
        data.get(key).cloned()
    }

    fn flush_data_v2(&mut self) -> Result<()> {
        let mut memtable = self.memtable.lock().map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("MemTable lock poisoned during flush: {}", e),
            )
        })?;
        let data_to_flush: BTreeMap<String, Value> = memtable.take_data();
        drop(memtable);

        let sorted = data_to_flush
            .into_iter()
            .collect::<BTreeMap<String, Value>>();

        if sorted.is_empty() {
            return Ok(());
        }

        let min_key = sorted.keys().next().cloned().unwrap();
        let max_key = sorted.keys().next_back().cloned().unwrap();

        create_dir_all("data").map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to create data dir: {}", e),
            )
        })?;

        fn sanitize_filename_part(s: &str) -> String {
            s.chars()
                .map(|c| {
                    if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                        c
                    } else {
                        '_'
                    }
                })
                .take(64)
                .collect()
        }

        let min_s = sanitize_filename_part(&min_key);
        let max_s = sanitize_filename_part(&max_key);

        let filename = format!("data/sstable_{}_to_{}.dat", min_s, max_s);
        let tmp_name = format!("{}.tmp", filename);
        {
            let mut tmp_file = File::create(&tmp_name).map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to create temp file {}: {}", tmp_name, e),
                )
            })?;

            serde_json::to_writer(&mut tmp_file, &sorted).map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to write data to {}: {}", tmp_name, e),
                )
            })?;

            tmp_file.sync_all().map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to sync temp file {}: {}", tmp_name, e),
                )
            })?;
        }

        std::fs::rename(&tmp_name, &filename).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to rename {} -> {}: {}", tmp_name, filename, e),
            )
        })?;

        if let Ok(dir) = File::open("data") {
            let _ = dir.sync_all();
        }

        let min_num = min_key
            .strip_prefix("key_")
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "min_key missing 'key_' prefix"))?
            .parse::<u64>()
            .map_err(|_| Error::new(ErrorKind::InvalidData, "min_key is not a valid u64"))?;

        let max_num = max_key
            .strip_prefix("key_")
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "max_key missing 'key_' prefix"))?
            .parse::<u64>()
            .map_err(|_| Error::new(ErrorKind::InvalidData, "max_key is not a valid u64"))?;

        self.range_map.push((min_num, max_num));

        Ok(())
    }

    fn get_sstable_filename(&self, key: &str) -> Option<String> {
        let id = key
            .strip_prefix("key_")
            .and_then(|s| s.parse::<u64>().ok())?;

        println!("{}", id);

        let mut lo: usize = 0;
        let mut hi: usize = self.range_map.len();

        println!("low : {}, high: {}", lo, hi);

        while lo < hi {
            let mid = (lo + hi) / 2;
            let (min, max) = self.range_map[mid];

            println!("Checking range: {} to {}", min, max);
            if id < min {
                hi = mid;
            } else if id > max {
                lo = mid + 1;
            } else {
                return Some(format!("data/sstable_key_{}_to_key_{}.dat", min, max));
            }
        }

        None
    }
}
