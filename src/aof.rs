// use std::fs::{File, OpenOptions};
// use std::io::{BufRead, BufReader, Result, Write};

// use crate::db::DB;
// // 
// pub struct Aof {
//     file: std::fs::File,
// }

// impl Aof {
//     pub fn new(path: &str) -> Result<Self> {
//         let file = OpenOptions::new().create(true).append(true).open(path)?;

//         Ok(Self { file })
//     }

//     pub fn write_set(&mut self, key: &str, value: &str) -> Result<()> {
//         writeln!(self.file, "SET {} {}", key, value)?;
//         self.file.flush()?;
//         Ok(())
//     }

//     pub fn write_del(&mut self, key: &str) -> Result<()> {
//         writeln!(self.file, "DEL {}", key)?;
//         self.file.flush()?;
//         Ok(())
//     }

//     pub fn replay(path: &str, db: &mut DB) -> Result<()> {
//         let file = File::open(path)?;
//         let reader = BufReader::new(file);

//         for line in reader.lines() {
//             let line = line?;
//             let parts: Vec<&str> = line.splitn(3, ' ').collect();

//             match parts.as_slice() {
//                 ["SET", key, value] => {
//                     db.set((*key).to_string(), (*value).to_string());
//                 }
//                 ["DEL", key] => {
//                     db.delete(key);
//                 }
//                 _ => {
//                     // ignore corrupted / unknown lines
//                 }
//             }
//         }

//         Ok(())
//     }
// }
