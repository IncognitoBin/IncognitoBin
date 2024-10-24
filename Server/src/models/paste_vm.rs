use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreatePasteRequest {
    pub(crate) title: String,
    pub(crate) signature: String,
    pub(crate) content: String,
    pub(crate) syntax: Option<String>,
    pub(crate) expire: Option<i32>,
    pub(crate) burn: Option<bool>,
}
#[derive(Serialize)]
pub struct CreatedPasteResponse {
    pub(crate) id: Uuid,
}
#[derive(Serialize)]
pub struct PasteResponse {
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) signature: String,
    pub(crate) syntax: Option<String>,
    pub(crate) expire: Option<i64>,
    pub(crate) views: i64,
}
