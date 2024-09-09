use anyhow::Result;
use futures::TryStreamExt;
use scylla::transport::session::Session;
use uuid::Uuid;
use crate::db::models::{PasteById, UserById, UserByToken};
use scylla::frame::value::Counter;

pub async fn get_user_by_id(session: &Session,userid:Uuid) -> Result<Option<UserById>> {
    let mut iter = session
        .query_iter("SELECT user_id, user_token, username FROM user_by_id where user_id = ? LIMIT 1;", (userid,))
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
pub async fn get_paste_by_id(session: &Session, paste_id: Uuid) -> Result<Option<PasteById>> {
    let mut iter = session
        .query_iter(
            "SELECT paste_id, title, content, syntax, password, encrypted, expire, burn, user_id
             FROM paste_by_id WHERE paste_id = ? LIMIT 1;",
            (paste_id,)
        )
        .await?
        .into_typed::<PasteById>();

    while let Some(paste) = iter.try_next().await? {
        return Ok(Some(paste));
    }

    Ok(None)
}
pub async fn get_pastes_by_userid(session: &Session, userid: Uuid) -> Result<Vec<Uuid>> {
    let mut iter = session
        .query_iter(
            "SELECT paste_id
             FROM pastes_by_user_id WHERE user_id = ?;",
            (userid,)
        )
        .await?
        .into_typed::<(Uuid,)>();

    let mut paste_ids = Vec::new();

    while let Some((paste_id,)) = iter.try_next().await? {
        paste_ids.push(paste_id);
    }

    Ok(paste_ids)
}
pub async fn get_view_count_by_paste_id(session: &Session, paste_id: Uuid) -> Result<Option<Counter>> {
    let mut iter = session
        .query_iter(
            "SELECT view_count
             FROM paste_view_counts WHERE paste_id = ?;",
            (paste_id,)
        )
        .await?
        .into_typed::<(Counter,)>();



    while let Some((views_count,)) = iter.try_next().await? {
        return Ok(Some(views_count));
    }
    Ok(None)
}
pub async fn increment_view_count_by_paste_id(session: &Session, paste_id: Uuid) -> Result<()> {
    session
        .query_unpaged(
            "UPDATE paste_view_counts
            SET view_count = view_count + 1
            WHERE paste_id = ?;",
            (paste_id,)
        )
        .await?;
    Ok(())
}
async fn insert_user_by_id(session: &Session, user: &UserById) -> Result<()> {
    session
        .query_unpaged(
            "INSERT INTO user_by_id (user_id, username, user_token) VALUES (?, ?, ?)",
            (user.user_id, &user.username, &user.user_token),
        )
        .await?;
    Ok(())
}

async fn insert_user_by_token(session: &Session, user: &UserByToken) -> Result<()> {
    session
        .query_unpaged(
            "INSERT INTO user_by_token (user_token, user_id) VALUES (?, ?)",
            (&user.user_token, user.user_id),
        )
        .await?;
    Ok(())
}