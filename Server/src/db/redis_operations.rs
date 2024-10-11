use redis::{Commands, RedisResult};
pub fn dequeue(con: &mut redis::Connection, queue_name: &str) -> RedisResult<Option<String>> {
    con.rpop(queue_name, None)
}