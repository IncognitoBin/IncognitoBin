use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreatePasteRequest {
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) syntax: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) encrypted: bool,
    pub(crate) expire: Option<i64>,
    pub(crate) burn: Option<bool>,
    pub(crate) user_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct CreatePasteResponse {
    pub(crate) paste_id: Uuid,
}