use std::collections::HashSet;
use std::time::Duration;
use redis::{Connection, RedisResult};
use crate::db::db_operations_imlp::{DbOperations, ScyllaDbOperations};
use crate::db::redis_operations::{queue_length, read_all};
use crate::utils::helpers::{generate_token, random_id};
use tokio::time::sleep;
use uuid::Uuid;
use crate::config::settings::{get_users_id_size, get_users_token_size};

pub async fn tokens_queue_handler(mut redis_con: Connection, db_ops: ScyllaDbOperations) {
    loop {
        let length = queue_length(&mut redis_con, "users_tokens").expect("Redis: Failed to get queue count");
        if length < get_users_token_size() {
            add_users_tokens_enqueue(&mut redis_con, "users_tokens", get_users_token_size() - length, &db_ops)
                .await
                .expect("Can't Insert To Tokens Queue");
        }
        sleep(Duration::from_secs(1)).await;
    }
}

pub async fn ids_queue_handler(mut redis_con: Connection, db_ops: ScyllaDbOperations) {
    loop {
        let length = queue_length(&mut redis_con, "users_ids").expect("Redis: Failed to get queue count");
        if length < get_users_id_size() {
            add_users_ids_enqueue(&mut redis_con, "users_ids", get_users_id_size() - length, &db_ops)
                .await
                .expect("Can't Insert To Ids Queue");
        }
        sleep(Duration::from_secs(1)).await;
    }
}

pub async fn add_users_tokens_enqueue(
    con: &mut Connection,
    queue_name: &str,
    size: usize,
    db_ops: &ScyllaDbOperations
) -> RedisResult<()> {
    let all_old_tokens = read_all(con, queue_name)?;
    let mut new_tokens = HashSet::new();
    while new_tokens.len() < size {
        let token = generate_token();
        let token_exists = db_ops.user_by_token_exist(&token)
            .await
            .expect("ScyllaDB: Can't Select Tokens");

        if !all_old_tokens.contains(&token.to_string()) && !token_exists {
            new_tokens.insert(token);
        }
    }
    let mut pipe = redis::pipe();
    for token in &new_tokens {
        pipe.lpush(queue_name, token.to_string());
    }
    pipe.query(con)?;
    Ok(())
}

pub async fn add_users_ids_enqueue(
    con: &mut Connection,
    queue_name: &str,
    size: usize,
    db_ops: &ScyllaDbOperations
) -> RedisResult<()> {
    let all_old_ids = read_all(con, queue_name)?;
    let mut new_ids = HashSet::new();
    while new_ids.len() < size {
        let id = random_id();
        let token_exists = db_ops.user_by_id_exist(&Uuid::from_u128(id)).await.expect("ScyllaDB: Can't Select Tokens");

        if !all_old_ids.contains(&id.to_string()) && !token_exists {
            new_ids.insert(id);
        }
    }
    let mut pipe = redis::pipe();
    for id in &new_ids {
        pipe.lpush(queue_name, id.to_string());
    }
    pipe.query(con)?;
    Ok(())
}
