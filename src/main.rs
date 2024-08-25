mod db;

use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::VecDeque;
use rusqlite::{Connection, Result};
use std::thread::spawn;
use crate::db::init_db;

const SPLIT_SIZE: u16 = 30000;
struct Chunk {
    id: u16,
    start: u128,
    end: u128,
    size: u8,
}
// 0: Increment
// 1: Update
struct DbOperation {
    chunk: Chunk,
    operation: u8,
}
static DB_UPDATE_QUEUE: Lazy<Mutex<VecDeque<DbOperation>>> = Lazy::new(|| Mutex::new(VecDeque::new()));
static CHUNKS: Lazy<Mutex<Vec<Chunk>>> = Lazy::new(|| Mutex::new(Vec::new()));
fn upgrade_chunk(chunk_to_index: usize) {
    let mut chunks = CHUNKS.lock().expect("Failed to lock CHUNKS mutex");
    let chunk = &mut chunks[chunk_to_index];
    if chunk.start == chunk.end {
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
    let mut db_update_queue = DB_UPDATE_QUEUE.lock().expect("Failed to lock DB QUEUE mutex");
    db_update_queue.push_back(DbOperation {
        operation:1,
        chunk: Chunk {
            id :chunk.id,
            start :chunk.start,
            end :chunk.end,
            size :chunk.size
        }
    })
}

fn init(start_id_size: u8) {
    let mut chunks = CHUNKS.lock().expect("Failed to lock CHUNKS mutex");
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
fn main() -> Result<()>{
    let conn = init_db("chunks.db")?;
    init(5);
    upgrade_chunk(0);
    Ok(())
}