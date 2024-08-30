use std::thread;
use std::time::Duration;

mod chunks;
const SPLIT_SIZE: u16 = 30000;
fn main() {

    match chunks::load() {
        Ok(()) => println!("Data loaded successfully!"),
        Err(e) => eprintln!("Failed to load data: {}", e),
    }
    chunks::upgrade_chunk(0);
    match chunks::store_chunks() {
        Ok(()) => println!("Saved"),
        Err(e) => eprintln!("Failed to store data: {}", e),
    }

    let handle = thread::spawn(|| {
        loop {
            chunks::store_chunks().expect("Can't Store The File!");
            thread::sleep(Duration::from_secs(15));
        }
    });
    handle.join().unwrap();
}
