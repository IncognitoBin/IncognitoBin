use std::fs::File;
use std::sync::{Mutex, RwLock};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use crate::SPLIT_SIZE;
use std::io::{ErrorKind, Result};
use std::io::BufReader;
#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Chunk {
    pub(crate) id: u16,
    pub(crate) start: u128,
    pub(crate) end: u128,
    pub(crate) size: u8,
}
pub(crate) static CHUNKS: Lazy<RwLock<Vec<Chunk>>> = Lazy::new(|| {
    RwLock::new(vec![])
});
pub fn load() -> Result<()> {
    let file = match File::open("data.json") {
        Ok(file) => { file },
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            let mut file = File::create("data.json")?;
            init(5);
            let chunks = CHUNKS.read().unwrap().clone();
            serde_json::to_writer(&mut file, &chunks)?;
            return Ok(())
        }
        Err(e) => return Err(e.into()), // Return other errors
    };
    let reader = BufReader::new(file);
    let chunks: Vec<Chunk> = serde_json::from_reader(reader)?;
    let mut chunks_lock = CHUNKS.write().unwrap();
    *chunks_lock = chunks;
    Ok(())
}
pub fn init(start_id_size: u8) {
    let mut chunks = CHUNKS.write().expect("Failed to lock CHUNKS mutex");
    let start: u128 = 62_u128.pow(start_id_size as u32 - 1);
    let end: u128 = 62_u128.pow(start_id_size as u32) - 1;
    let chunk_size: u128 = (end - start) / SPLIT_SIZE as u128;
    for i in 0..SPLIT_SIZE {
        chunks.push(Chunk {
            id: i,
            start: start + chunk_size * i as u128,
            end: if i == SPLIT_SIZE - 1 {
                end
            } else {
                start + chunk_size * i as u128 + chunk_size - 1
            },
            size: start_id_size,
        });
    }
}
