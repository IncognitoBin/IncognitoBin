use redis::{Commands, RedisResult};

pub fn queue_length(con: &mut redis::Connection, queue_name: &str) -> RedisResult<usize> {
    con.llen(queue_name)
}
pub fn read_all(con: &mut redis::Connection, queue_name: &str) -> RedisResult<Vec<String>> {
    let len: usize = con.llen(queue_name)?;
    con.lrange(queue_name, 0, len as isize - 1)
}
