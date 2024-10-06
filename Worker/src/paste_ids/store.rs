use std::fs::File;
use std::io::{BufWriter, BufReader, ErrorKind, Result, Write};
use crate::paste_ids::ChunkManager;
use crate::paste_ids::manager::{init, MANAGER};
pub fn store_chunks() -> Result<()> {
    let file = File::create("data.json")?;
    let mut writer = BufWriter::new(file);
    let manager = MANAGER.read().unwrap();
    serde_json::to_writer(&mut writer, &*manager)?;
    writer.flush()?;
    Ok(())
}

pub fn load() -> Result<()> {
    let file = match File::open("data.json") {
        Ok(file) => file,
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
