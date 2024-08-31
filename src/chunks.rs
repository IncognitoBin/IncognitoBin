use std::fs::File;
use std::sync::{RwLock};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use crate::SPLIT_SIZE;
use std::io::{BufWriter, ErrorKind, Result, Write};
use std::io::BufReader;
use std::collections::VecDeque;
use chrono::{Duration, Utc};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Chunk {
    pub(crate) id: u16,
    pub(crate) start: u128,
    pub(crate) end: u128,
    pub(crate) size: u8,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChunkManager {
    pub(crate) chunks: Vec<Chunk>,
    pub(crate) index: u16,
    pub(crate) expired: VecDeque<UsedID>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UsedID {
    pub(crate) id: u128,
    pub(crate) available: i64,
}
pub(crate) static MANAGER: Lazy<RwLock<ChunkManager>> = Lazy::new(|| {
    RwLock::new(ChunkManager {
        chunks: Vec::new(),
        index: 0,
        expired: VecDeque::new(),
    })
});

pub fn load() -> Result<()> {
    let file = match File::open("data.json") {
        Ok(file) => { file }
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            let mut file = File::create("data.json")?;
            init(5);
            let manager = MANAGER.read().unwrap().clone();
            serde_json::to_writer(&mut file, &manager)?;
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    };
    let reader = BufReader::new(file);
    let chunks: ChunkManager = serde_json::from_reader(reader)?;
    let mut chunks_lock = MANAGER.write().unwrap();
    *chunks_lock = chunks;
    Ok(())
}
pub fn init(start_id_size: u8) {
    let mut manager = MANAGER.write().expect("Failed to lock CHUNKS mutex");
    let start: u128 = 62_u128.pow(start_id_size as u32 - 1);
    let end: u128 = 62_u128.pow(start_id_size as u32) - 1;
    let chunk_size: u128 = (end - start) / SPLIT_SIZE as u128;
    for i in 0..SPLIT_SIZE {
        manager.chunks.push(Chunk {
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
pub fn store_chunks() -> Result<()> {
    let file = File::create("data.json")?;
    let mut writer = BufWriter::new(file);
    let manager = MANAGER.read().unwrap();
    serde_json::to_writer(&mut writer, &*manager)?;
    writer.flush()?;
    Ok(())
}
fn get_id_from_expired_id() -> u128{
    let mut manager = MANAGER.write().unwrap();
    if let Some(front) = manager.expired.pop_front() {
        if Utc::now().timestamp() >= front.available {
            return front.id;
        }
    }
    0
}
fn increment_index(){
    let mut manager = MANAGER.write().unwrap();
    manager.index+=1;
    if manager.index>=SPLIT_SIZE {
        manager.index=0
    }
}
fn update_chunk_start(index :u16) {
    let mut manager = MANAGER.write().unwrap(); // Get a mutable reference with `write`
    if let Some(chunk) = manager.chunks.get_mut(index as usize) { // Use `get_mut` for a mutable reference
        chunk.start += 1; // Mutate the field directly
        if chunk.start==chunk.end && chunk.size < 21 {
            chunk.size += 1;
            let start: u128 = 62_u128.pow(chunk.size as u32 - 1);
            let end: u128 = 62_u128.pow(chunk.size as u32) - 1;
            let chunk_size: u128 = (end - start) / SPLIT_SIZE as u128;
            chunk.start = start + chunk_size * chunk.size as u128;
            chunk.end = if chunk.size as u16 == SPLIT_SIZE - 1 {
                end
            } else {
                start + chunk_size * chunk.size as u128 + chunk_size - 1
            };
        }
    }
}
fn get_id_from_chunk() -> (u128,u16){
    let manager = MANAGER.read().unwrap();
    if let Some(value) = manager.chunks.get(manager.index as usize){
        return (value.start,value.id)
    }
    (0,0)
}

pub fn retrieve_id() -> u128 {
    let id=get_id_from_expired_id();
    if id!=0 {
        return id;
    }
    increment_index();
    let (id,index)=get_id_from_chunk();
    update_chunk_start(index);
    id
}
pub fn add_id(id: u128) -> u128 {
    let mut manager = MANAGER.write().unwrap();
    manager.expired.push_back(
        UsedID {
            id,
            available:
            (Utc::now() + Duration::days(30))
                .timestamp(),
        });
    id
}