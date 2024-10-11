use scylla::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub(crate) struct UserById {
    pub(crate) user_id: Uuid,
    pub(crate) user_token: String,
}



