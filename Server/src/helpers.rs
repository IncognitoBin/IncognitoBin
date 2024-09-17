use actix_web::HttpRequest;
use anyhow::Context;
use uuid::Uuid;
use crate::config::Config;
use crate::db::paste_db_operations::PasteDbOperations;
use crate::db::scylla_db_operations::ScyllaDbOperations;


pub fn is_valid_bcrypt_hash(hash: String) -> Option<u8> {
    if hash.len() != 60 {
        return None;
    }
    if hash.starts_with("$2b$") {
        let parts: Vec<&str> = hash.split('$').collect();
        if parts.len() > 2 {
            if let Ok(cost) = parts[2].parse::<u8>() {
                return Some(cost);
            }
        }
    }
    None
}
pub async fn generate_unique_id() -> anyhow::Result<Uuid> {
    let response = reqwest::get("http://localhost:8080/id")
        .await
        .context("Failed to connect to ID generation service")?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("ID generation service returned an error: {}", response.status()));
    }

    let id: u128 = response.text().await
        .context("Failed to read response from ID generation service")?
        .parse()
        .context("Failed to parse ID as u128")?;

    Ok(Uuid::from_u128(id))
}
pub async fn extract_user_id(req: &HttpRequest, db: &ScyllaDbOperations, config: &Config) -> Option<Uuid> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_token) = auth_header.to_str() {
            if auth_token.len() == config.token_size as usize {
                if let Ok(Some(user_id)) = db.get_userid_by_token(auth_token).await {
                    return Some(user_id);
                }
            }
        }
    }
    None
}
