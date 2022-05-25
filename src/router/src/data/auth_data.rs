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
