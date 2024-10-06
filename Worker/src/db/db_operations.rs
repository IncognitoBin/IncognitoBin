use anyhow::Result;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait DbOperations {
    async fn user_by_id_exist(&self, userid: &Uuid) -> Result<bool>;
    async fn user_by_token_exist(&self, token: &str) -> Result<bool>;

}