use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use scylla::FromRow;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub(crate) struct UserById {
    pub(crate) user_id: Uuid,
    pub(crate) user_token: String,
}

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub(crate) struct UserByToken {
    pub(crate) user_token: String,
    pub(crate) user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub(crate) struct PasteById {
    pub(crate) paste_id: Uuid,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) syntax: Option<String>,
    pub(crate) password: bool,
    pub(crate) expire: Option<DateTime<Utc>>,
    pub(crate) burn: bool,
    pub(crate) user_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize,FromRow)]
struct ExpireDate {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
}
