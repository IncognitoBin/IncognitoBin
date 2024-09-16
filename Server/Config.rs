use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub bcrypt_rounds: u8,
    pub max_title_length: u8,
    pub max_content_kb: u16,
    pub token_size: u8,
    pub max_syntax_length: u8
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        Ok(Config {
            bcrypt_rounds: env::var("BCRYPT_ROUNDS")?.parse().unwrap_or(12),
            max_title_length: env::var("MAX_TITLE_LENGTH")?.parse().unwrap_or(20),
            max_content_kb: env::var("MAX_CONTENT_KB")?.parse().unwrap_or(10000),
            token_size: env::var("TOKEN_SIZE")?.parse().unwrap_or(150),
            max_syntax_length: env::var("MAX_SYNTAX_LENGTH")?.parse().unwrap_or(20),
        })
    }
}
