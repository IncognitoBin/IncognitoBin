use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreatePasteRequest {
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) syntax: Option<String>,
    pub(crate) password: bool,
    pub(crate) expire: Option<i32>,
    pub(crate) burn: Option<bool>,
}
#[derive(Deserialize)]
pub struct UserLoginRequest {
    pub(crate) user_id: u128,
}
#[derive(Serialize)]
pub struct CreatePasteResponse {
    pub(crate) paste_id: Uuid,
}