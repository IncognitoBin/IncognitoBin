use std::sync::RwLock;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use crate::config::settings::get_split_size;

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

pub fn init(start_id_size: u8) {
    let mut manager = MANAGER.write().expect("Failed to lock CHUNKS mutex");
    let start: u128 = 62_u128.pow(start_id_size as u32 - 1);
    let end: u128 = 62_u128.pow(start_id_size as u32) - 1;
    let chunk_size: u128 = (end - start) / get_split_size() as u128;
    for i in 0..get_split_size() {
        manager.chunks.push(Chunk {
            id: i,
            start: start + chunk_size * i as u128,
            end: if i == get_split_size() - 1 {
                end
            } else {
                start + chunk_size * i as u128 + chunk_size - 1
            },
            size: start_id_size,
        });
    }
}
