use std::time::Duration;
use redis::{Connection, RedisResult};
use crate::paste_ids::retrieve_id;
use crate::db::redis_operations::queue_length;
use tokio::time::sleep;
use crate::config::settings::get_pastes_id_size;

pub async fn pastes_ids_handler(mut redis_con: Connection){
    loop {
        let length = queue_length(&mut redis_con, "paste_ids").expect("Redis: Failed to get queue count");
        if length < get_pastes_id_size() {
            add_paste_ids_enqueue(&mut redis_con, "paste_ids", get_pastes_id_size()-length).expect("Redis: Can't Insert To Queue");
        }
        sleep(Duration::from_secs(1)).await;
    }
}
pub fn add_paste_ids_enqueue(con: &mut Connection, queue_name: &str,size:usize) -> RedisResult<()> {
    let mut pipe = redis::pipe();
    for _ in 0..size  {
        pipe.lpush(queue_name, retrieve_id().to_string());
    }
    pipe.query(con)?;
    Ok(())
}