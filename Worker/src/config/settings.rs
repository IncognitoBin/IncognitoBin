use std::env;
use once_cell::sync::Lazy;
static SPLIT_SIZE: Lazy<u16> = Lazy::new(|| {
    env::var("SPLIT_SIZE")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(10000)
});
static PASTES_ID_SIZE: Lazy<usize> = Lazy::new(|| {
    env::var("PASTES_ID_SIZE")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(500000)
});
static USERS_ID_SIZE: Lazy<usize> = Lazy::new(|| {
    env::var("USERS_ID_SIZE")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(10000)
});
static USERS_TOKEN_SIZE: Lazy<usize> = Lazy::new(|| {
    env::var("USERS_TOKEN_SIZE")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(10000)
});
pub fn get_split_size() -> u16 {
    *SPLIT_SIZE
}
pub fn get_pastes_id_size() -> usize {
    *PASTES_ID_SIZE
}
pub fn get_users_id_size() -> usize {
    *USERS_ID_SIZE
}
pub fn get_users_token_size() -> usize {
    *USERS_TOKEN_SIZE
}