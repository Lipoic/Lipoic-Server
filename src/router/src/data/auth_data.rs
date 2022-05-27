use database::model::auth::user::UserMode;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Auth {
    pub(crate) url: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub(crate) token: String,
}

#[derive(FromForm)]
pub struct LoginFromData {
    pub(crate) password: String,
    pub(crate) email: String,
}

#[derive(FromForm)]
pub struct SignUp {
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) modes: Json<Vec<UserMode>>,
}
