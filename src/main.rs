use std::sync::Mutex;
use lazy_static::lazy_static;

static SPLIT_SIZE: u16 = 30000;
struct Chunk {
    id: u16,
    start: u128,
    end: u128,
    size: u8,
}
lazy_static! {
    static ref CHUNKS: Mutex<Vec<Chunk>> = Mutex::new(Vec::new());
}
fn upgrade_chunk(chunk_to_index: usize) {
    let mut chunks = CHUNKS.lock().unwrap();
    if(chunks[chunk_to_index].start==chunks[chunk_to_index].end){
        chunks[chunk_to_index].size += 1;
        let start: u128 = 62_u128.pow(chunks[chunk_to_index].size as u32 -1);
        let end: u128 = 62_u128.pow(chunks[chunk_to_index].size as u32) - 1;
        let chunk_size: u128 = (end - start) / SPLIT_SIZE as u128;
        chunks[chunk_to_index].start = start + chunk_size * chunks[chunk_to_index].size as u128;
        chunks[chunk_to_index].end = if chunks[chunk_to_index].size as u16 == SPLIT_SIZE - 1 {
            end
        } else {
            start + chunk_size * chunks[chunk_to_index].size as u128 + chunk_size - 1
        };
    }
}
fn init(start_id_size: u8) {
    let mut chunks = CHUNKS.lock().unwrap();
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
// 13_537_086_546_263_552    (9 characters)
// 218_340_105_584_896       (8 characters)
// 56_800_235_584            (7 characters)
// 916_132_832               (6 characters)
// 14_776_336                (5 characters)
// 238_328                   (4 characters)
// 3_844                     (3 characters)
fn main() {
    // Minimum 3
    init(5);
    upgrade_chunk(0);

}