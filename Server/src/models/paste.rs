use chrono::{DateTime, Utc};
use scylla::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub(crate) struct PasteById {
    pub(crate) paste_id: Uuid,
    pub(crate) title: String,
    pub(crate) signature: String,
    pub(crate) content: String,
    pub(crate) syntax: Option<String>,
    pub(crate) expire: Option<DateTime<Utc>>,
    pub(crate) burn: bool,
    pub(crate) user_id: Option<Uuid>,
}
#[derive(Debug, Serialize, Deserialize,FromRow)]
pub(crate) struct PasteInfoById {
    pub(crate) expire: Option<DateTime<Utc>>,
    pub(crate) burn: bool,
}