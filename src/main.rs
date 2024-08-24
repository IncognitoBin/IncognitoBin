use std::sync::Mutex;
use lazy_static::lazy_static;

static SPLIT_SIZE:u16=1000;
struct Chunk{
    id :u16,
    start:u128,
    end:u128,
    size :u8,
}
lazy_static! {
    static ref CHUNKS: Mutex<Vec<Chunk>> = Mutex::new(Vec::new());
}
fn init(start_id_size:u8){
    let mut chunks = CHUNKS.lock().unwrap();
    let start :u128=62_u128.pow(start_id_size as u32 -1);
    let end :u128=62_u128.pow(start_id_size as u32)-1;
    let chunk_size:u128=(end-start)/SPLIT_SIZE as u128;
    for i in 0..SPLIT_SIZE{
        let mut chunk = Chunk{
            id:i,
            start: start+ chunk_size *i as u128,
            end: 0,
            size: start_id_size ,
        };
        chunk.end = if i==SPLIT_SIZE- 1 {
            end
        }else{
            chunk.start+ chunk_size -1
        };
        chunks.push(chunk);
    }

}
fn main()  {
    // Minimum 3
    init(5);
}