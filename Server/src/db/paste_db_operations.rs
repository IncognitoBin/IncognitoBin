use anyhow::Result;
use uuid::Uuid;
use scylla::frame::value::Counter;
use crate::models::paste::{PasteById, PasteInfoById};
use crate::models::user::UserById;

#[async_trait::async_trait]
pub trait PasteDbOperations {
    async fn get_user_by_id(&self, userid: Uuid) -> Result<Option<UserById>>;
    async fn get_userid_by_token(&self, token: &str) -> Result<Option<Uuid>>;
    async fn get_paste_by_id(&self, paste_id: Uuid) -> Result<Option<PasteById>>;
    async fn get_paste_info_by_id(&self, paste_id: Uuid) -> Result<Option<PasteInfoById>>;
    async fn get_pastes_by_userid(&self, userid: Uuid) -> Result<Vec<Uuid>>;
    async fn check_paste_by_userid(&self, userid: &Uuid,paste_id:&Uuid) -> Result<bool>;
    async fn get_view_count_by_paste_id(&self, paste_id: Uuid) -> Result<Option<Counter>>;
    async fn increment_view_count_by_paste_id(&self, paste_id: Uuid) -> Result<()>;
    async fn insert_user_by_id(&self, user: &UserById) -> Result<()>;
    async fn insert_paste(&self, paste: &PasteById,duration:i32) -> Result<()>;
    async fn insert_paste_by_user_id(&self, user_id: Uuid, paste_id: Uuid, duration: i32) -> Result<()>;
    async fn delete_paste_by_id(&self, paste_id: &Uuid) -> Result<()>;
    async fn delete_paste_by_user_id(&self, paste_id:&Uuid, user: &Uuid) -> Result<()>;
    async fn delete_user_token(&self, token: String) -> Result<()>;
    async fn execute_update_token_operations(&self, old_token: String, new_token: String,user_id:&Uuid) -> Result<()>;
}