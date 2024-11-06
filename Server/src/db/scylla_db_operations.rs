use crate::db::paste_db_operations::PasteDbOperations;
use crate::models::paste::{PasteById, PasteInfoById};
use crate::models::user::UserById;
use anyhow::{Context, Result};
use futures::TryStreamExt;
use scylla::batch::Batch;
use scylla::frame::value::Counter;
use scylla::CachingSession;
use std::sync::Arc;
use uuid::Uuid;

pub struct ScyllaDbOperations {
    session: Arc<CachingSession>,
}

impl ScyllaDbOperations {
    pub fn new(session: Arc<CachingSession>) -> Self {
        Self { session }
    }
}
// TODO: Batch Query For Create Paste -> https://github.com/scylladb/scylla-rust-driver/blob/main/docs/source/queries/batch.md
#[async_trait::async_trait]
impl PasteDbOperations for ScyllaDbOperations {
    async fn get_user_by_id(&self, userid: Uuid) -> Result<Option<UserById>> {
        let mut iter = self
            .session
            .execute_iter(
                "SELECT user_id, user_token FROM user_by_id where user_id = ? LIMIT 1;",
                (userid,),
            )
            .await?
            .into_typed::<UserById>();
        while let Some(user) = iter.try_next().await? {
            return Ok(Some(user));
        }
        Ok(None)
    }

    async fn get_userid_by_token(&self, token: &str) -> Result<Option<Uuid>> {
        let mut iter = self
            .session
            .execute_iter(
                "SELECT user_id FROM user_by_token WHERE user_token = ? LIMIT 1;",
                (token,),
            )
            .await?
            .into_typed::<(Uuid,)>();

        while let Some((userid,)) = iter.try_next().await? {
            return Ok(Some(userid));
        }

        Ok(None)
    }
    async fn get_paste_by_id(&self, paste_id: Uuid) -> Result<Option<PasteById>> {
        let mut iter = self
            .session
            .execute_iter(
                "SELECT paste_id, title, signature, content, syntax, expire, burn, user_id
             FROM paste_by_id WHERE paste_id = ? LIMIT 1;",
                (paste_id,),
            )
            .await?
            .into_typed::<PasteById>();

        while let Some(paste) = iter.try_next().await? {
            return Ok(Some(paste));
        }

        Ok(None)
    }
    async fn get_paste_info_by_id(&self, paste_id: Uuid) -> Result<Option<PasteInfoById>> {
        let mut iter = self
            .session
            .execute_iter(
                "SELECT expire, burn
             FROM paste_by_id WHERE paste_id = ? LIMIT 1;",
                (paste_id,),
            )
            .await?
            .into_typed::<PasteInfoById>();

        while let Some(paste) = iter.try_next().await? {
            return Ok(Some(paste));
        }

        Ok(None)
    }
    async fn get_pastes_by_userid(&self, userid: Uuid) -> Result<Vec<Uuid>> {
        let mut iter = self
            .session
            .execute_iter(
                "SELECT paste_id
             FROM pastes_by_user_id WHERE user_id = ?;",
                (userid,),
            )
            .await?
            .into_typed::<(Uuid,)>();

        let mut paste_ids = Vec::new();

        while let Some((paste_id,)) = iter.try_next().await? {
            paste_ids.push(paste_id);
        }

        Ok(paste_ids)
    }
    async fn get_view_count_by_paste_id(&self, paste_id: Uuid) -> Result<Option<Counter>> {
        let mut iter = self
            .session
            .execute_iter(
                "SELECT view_count
             FROM paste_view_counts WHERE paste_id = ?;",
                (paste_id,),
            )
            .await?
            .into_typed::<(Counter,)>();

        while let Some((views_count,)) = iter.try_next().await? {
            return Ok(Some(views_count));
        }
        Ok(None)
    }
    async fn increment_view_count_by_paste_id(&self, paste_id: Uuid) -> Result<()> {
        self.session
            .execute_unpaged(
                "UPDATE paste_view_counts
            SET view_count = view_count + 1
            WHERE paste_id = ?;",
                (paste_id,),
            )
            .await?;
        Ok(())
    }

    async fn insert_user_by_id(&self, user: &UserById) -> Result<()> {
        self.session
            .execute_unpaged(
                "INSERT INTO user_by_id (user_id, user_token) VALUES (?, ?)",
                (user.user_id, &user.user_token),
            )
            .await?;
        Ok(())
    }
    async fn insert_paste(&self, paste: &PasteById, duration: i32) -> Result<()> {
        let query = "INSERT INTO paste_by_id (
        paste_id, title, signature, content, syntax, expire, burn, user_id
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?) USING TTL ?";

        self.session
            .execute_unpaged(
                query,
                (
                    paste.paste_id,
                    &paste.title,
                    &paste.signature,
                    &paste.content,
                    &paste.syntax,
                    paste.expire,
                    paste.burn,
                    paste.user_id,
                    duration,
                ),
            )
            .await?;

        Ok(())
    }

    async fn insert_paste_by_user_id(
        &self,
        user_id: Uuid,
        paste_id: Uuid,
        duration: i32,
    ) -> Result<()> {
        let query = "INSERT INTO pastes_by_user_id (user_id, paste_id) VALUES (?, ?) USING TTL ?";
        self.session
            .execute_unpaged(query, (user_id, paste_id, duration))
            .await?;
        Ok(())
    }
    async fn delete_paste_by_id(&self, paste_id: &Uuid) -> Result<()> {
        self.session
            .execute_unpaged("DELETE FROM paste_by_id WHERE paste_id = ?", (paste_id,))
            .await?;
        Ok(())
    }
    async fn check_paste_by_userid(&self, userid: &Uuid, paste_id: &Uuid) -> Result<bool> {
        let mut iter = self
            .session
            .execute_iter(
                "SELECT paste_id
             FROM pastes_by_user_id WHERE user_id = ? and paste_id = ?;",
                (userid, paste_id),
            )
            .await?
            .into_typed::<(Uuid,)>();

        while let Some((_,)) = iter.try_next().await? {
            return Ok(true);
        }
        Ok(false)
    }
    async fn delete_paste_by_user_id(&self, paste_id: &Uuid, user_id: &Uuid) -> Result<()> {
        let mut batch: Batch = Default::default();
        // paste_by_id
        batch.append_statement("DELETE FROM paste_by_id WHERE paste_id = ?");
        // pastes_by_user_id
        batch.append_statement("DELETE FROM pastes_by_user_id WHERE user_id = ? and paste_id = ?");

        let prepared_batch: Batch = self.session.prepare_batch(&batch).await?;
        let batch_values = ((paste_id,), (user_id, paste_id));
        self.session.batch(&prepared_batch, batch_values).await?;
        Ok(())
    }
    async fn delete_user_token(&self, token: String) -> Result<()> {
        let query = "DELETE FROM user_by_token WHERE user_token = ?";
        self.session.execute_unpaged(query, (token,)).await?;
        Ok(())
    }
    async fn execute_update_token_operations(
        &self,
        old_token: String,
        new_token: String,
        user_id: &Uuid,
    ) -> Result<()> {
        let mut batch: Batch = Default::default();
        batch.append_statement("DELETE FROM user_by_token WHERE user_token = ?");
        batch.append_statement("INSERT INTO user_by_token (user_token, user_id) VALUES (?, ?)");
        batch.append_statement("UPDATE user_by_id SET user_token = ? WHERE user_id = ?");

        let prepared_batch: Batch = self.session.prepare_batch(&batch).await?;
        let batch_values = (
            (old_token,),
            (new_token.clone(), user_id),
            (new_token.clone(), user_id),
        );

        self.session
            .batch(&prepared_batch, batch_values)
            .await
            .context("Failed to execute batch")
            .expect("TODO: panic message");
        Ok(())
    }
}
