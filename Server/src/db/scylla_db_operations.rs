use scylla::Session;
use std::sync::Arc;
use futures::TryStreamExt;
use uuid::Uuid;
use anyhow::Result;
use scylla::frame::value::Counter;
use crate::db::models::{PasteById, UserById, UserByToken};
use crate::db::paste_db_operations::PasteDbOperations;
pub struct ScyllaDbOperations {
    session: Arc<Session>,
}

impl ScyllaDbOperations {
    pub fn new(session: Arc<Session>) -> Self {
        Self { session }
    }
}

#[async_trait::async_trait]
impl PasteDbOperations for ScyllaDbOperations {
    async fn get_user_by_id(&self, userid: Uuid) -> Result<Option<UserById>> {
        let mut iter = self.session
            .query_iter("SELECT user_id, user_token, username FROM user_by_id where user_id = ? LIMIT 1;", (userid,))
            .await?
            .into_typed::<UserById>();
        while let Some(user) = iter.try_next().await? {
            return Ok(Some(user));
        }
        Ok(None)
    }

    async fn get_userid_by_token(&self, token: &str) -> Result<Option<Uuid>> {
        let mut iter = self.session
            .query_iter("SELECT user_id FROM user_by_token WHERE user_token = ? LIMIT 1;", (token,))
            .await?
            .into_typed::<(Uuid,)>();

        while let Some((userid, )) = iter.try_next().await? {
            return Ok(Some(userid));
        }

        Ok(None)
    }
    async fn get_paste_by_id(&self, paste_id: Uuid) -> Result<Option<PasteById>> {
        let mut iter = self.session
            .query_iter(
                "SELECT paste_id, title, content, syntax, password, encrypted, expire, burn, user_id
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
    async fn get_pastes_by_userid(&self, userid: Uuid) -> Result<Vec<Uuid>> {
        let mut iter = self.session
            .query_iter(
                "SELECT paste_id
             FROM pastes_by_user_id WHERE user_id = ?;",
                (userid,),
            )
            .await?
            .into_typed::<(Uuid,)>();

        let mut paste_ids = Vec::new();

        while let Some((paste_id, )) = iter.try_next().await? {
            paste_ids.push(paste_id);
        }

        Ok(paste_ids)
    }
    async fn get_view_count_by_paste_id(&self, paste_id: Uuid) -> Result<Option<Counter>> {
        let mut iter = self.session
            .query_iter(
                "SELECT view_count
             FROM paste_view_counts WHERE paste_id = ?;",
                (paste_id,),
            )
            .await?
            .into_typed::<(Counter,)>();


        while let Some((views_count, )) = iter.try_next().await? {
            return Ok(Some(views_count));
        }
        Ok(None)
    }
    async fn increment_view_count_by_paste_id(&self, paste_id: Uuid) -> Result<()> {
        self.session
            .query_unpaged(
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
            .query_unpaged(
                "INSERT INTO user_by_id (user_id, username, user_token) VALUES (?, ?, ?)",
                (user.user_id, &user.username, &user.user_token),
            )
            .await?;
        Ok(())
    }

    async fn insert_user_by_token(&self, user: &UserByToken) -> Result<()> {
        self.session
            .query_unpaged(
                "INSERT INTO user_by_token (user_token, user_id) VALUES (?, ?)",
                (&user.user_token, user.user_id),
            )
            .await?;
        Ok(())
    }
    async fn insert_paste(&self, paste: &PasteById) -> Result<()> {
        let query = "INSERT INTO paste_by_id (
        paste_id, title, content, syntax, password, encrypted, expire, burn, user_id
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";

        self.session
            .query_unpaged(
                query,
                (
                    paste.paste_id,
                    &paste.title,
                    &paste.content,
                    &paste.syntax,
                    &paste.password,
                    paste.encrypted,
                    paste.expire,
                    paste.burn,
                    paste.user_id,
                ),
            )
            .await?;

        Ok(())
    }

    async fn insert_paste_by_user_id(&self, user_id: Uuid, paste_id: Uuid) -> Result<()> {
        let query = "INSERT INTO pastes_by_user_id (user_id, paste_id) VALUES (?, ?)";
        self.session
            .query_unpaged(query, (user_id, paste_id))
            .await?;
        Ok(())
    }

}