use chrono::{Duration, Utc};
use crate::chunks::constants::SPLIT_SIZE;
use crate::chunks::manager::{UsedID, MANAGER};

fn get_id_from_expired_id() -> u128 {
    let mut manager = MANAGER.write().unwrap();
    if let Some(front) = manager.expired.pop_front() {
        if Utc::now().timestamp() >= front.available {
            return front.id;
        }
    }
    0
}

fn increment_index() {
    let mut manager = MANAGER.write().unwrap();
    manager.index += 1;
    if manager.index >= SPLIT_SIZE {
        manager.index = 0;
    }
}

fn update_chunk_start(index: u16) {
    let mut manager = MANAGER.write().unwrap();
    if let Some(chunk) = manager.chunks.get_mut(index as usize) {
        chunk.start += 1; // Mutate the field directly
        if chunk.start == chunk.end && chunk.size < 21 {
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

fn get_id_from_chunk() -> (u128, u16) {
    let manager = MANAGER.read().unwrap();
    if let Some(value) = manager.chunks.get(manager.index as usize) {
        return (value.start, value.id);
    }
    (0, 0)
}

pub fn retrieve_id() -> u128 {
    let id = get_id_from_expired_id();
    if id != 0 {
        return id;
    }
    increment_index();
    let (id, index) = get_id_from_chunk();
    update_chunk_start(index);
    id
}

pub fn add_id(id: u128) -> u128 {
    let mut manager = MANAGER.write().unwrap();
    manager.expired.push_back(UsedID {
        id,
        available: (Utc::now() + Duration::days(30)).timestamp(),
    });
    id
}
