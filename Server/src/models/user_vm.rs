use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserLoginRequest {
    pub(crate) user_id: u128,
}