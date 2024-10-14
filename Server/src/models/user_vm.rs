use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UserLoginRequest {
    pub(crate) id: u128,
}
#[derive(Serialize)]
pub struct CreatedUserResponse {
    pub(crate) id: Uuid,
}
#[derive(Serialize)]
pub struct UserLoginResponse {
    pub(crate) token: String,
}