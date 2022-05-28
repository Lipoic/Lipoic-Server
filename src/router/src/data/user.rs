use rocket::serde::{Serialize};
use database::model::auth::user::{ConnectAccount, UserMode};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo {
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) modes: Vec<UserMode>,
    pub(crate) connects: Vec<ConnectAccount>,
}