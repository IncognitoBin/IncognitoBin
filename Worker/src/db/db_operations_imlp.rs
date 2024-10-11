use scylla::Session;
use std::sync::Arc;
use uuid::Uuid;
use anyhow::Result;
use futures::TryStreamExt;

pub use crate::db::db_operations::DbOperations;
pub struct ScyllaDbOperations {
    session: Arc<Session>,
}

impl ScyllaDbOperations {
    pub fn new(session: Arc<Session>) -> Self {
        Self { session }
    }
}
#[async_trait::async_trait]
impl DbOperations for ScyllaDbOperations {
    async fn user_by_id_exist(&self, userid: &Uuid) -> Result<bool> {
        let mut iter = self.session
            .query_iter("SELECT user_id FROM incognitobin_keyspace.user_by_id where user_id = ? LIMIT 1;", (userid,))
            .await?
            .into_typed::<(Uuid,)>();
        while let Some((_, )) = iter.try_next().await? {
            return Ok(true);
        }
        Ok(false)
    }

    async fn user_by_token_exist(&self, token: &str) -> Result<bool> {
        let mut iter = self.session
            .query_iter("SELECT user_id FROM incognitobin_keyspace.user_by_token WHERE user_token = ? LIMIT 1;", (token,))
            .await?
            .into_typed::<(Uuid,)>();

        while let Some((_, )) = iter.try_next().await? {
            return Ok(true);
        }
        Ok(false)
    }


}