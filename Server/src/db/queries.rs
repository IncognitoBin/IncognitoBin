use anyhow::Result;
use futures::TryStreamExt;
use scylla::transport::session::Session;
use uuid::Uuid;
use crate::db::models::UserById;

pub async fn get_user_by_id(session: &Session,id:Uuid) -> Result<Option<UserById>> {
    let mut iter = session
        .query_iter("SELECT user_id, user_token, username FROM user_by_id where user_id = ? LIMIT 1;", (id,))
        .await?
        .into_typed::<UserById>();
    while let Some(user) = iter.try_next().await? {
        return Ok(Some(user))
    }
    Ok(None)
}

pub async fn get_userid_by_token(session: &Session, token: &str) -> Result<Option<Uuid>> {
    let mut iter = session
        .query_iter("SELECT user_id FROM user_by_token WHERE user_token = ? LIMIT 1;", (token,))
        .await?
        .into_typed::<(Uuid,)>();

    while let Some((userid,)) = iter.try_next().await? {
      return Ok(Some(userid))
    }

    Ok(None)
}
