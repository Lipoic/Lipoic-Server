use crate::data::code::Code;
use crate::data::response::Response;
use crate::Config;
use database::model::auth::user::UserMode;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{Request, State};
use util::jwt::verify_token;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthUrl {
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    /// Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub(crate) exp: usize,
    pub(crate) username: String,
    pub(crate) verified_email: bool,
    pub(crate) id: String,
    pub(crate) modes: Vec<UserMode>,
}

#[doc(hidden)]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginUserData {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) verified_email: bool,
    pub(crate) modes: Vec<UserMode>,
}

#[derive(FromForm)]
pub struct EditUserData {
    pub(crate) username: Option<String>,
    pub(crate) is_student: Option<bool>,
    pub(crate) is_teacher: Option<bool>,
    pub(crate) is_parents: Option<bool>,
}

pub type AuthError = Unauthorized<Json<Response<String>>>;

impl LoginUserData {
    fn unauthorized() -> Outcome<Self, AuthError> {
        Outcome::Failure((
            Status::Unauthorized,
            Unauthorized(Some(Response::new(Code::AuthError, None))),
        ))
    }
}

#[rocket::async_trait]
#[doc(hidden)]
impl<'r> FromRequest<'r> for LoginUserData {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match request.headers().get_one("Authorization") {
            Some(v) => v,
            None => {
                return LoginUserData::unauthorized();
            }
        };

        let token_info = token.split(' ').collect::<Vec<_>>();
        if token_info.len() < 2 {
            return LoginUserData::unauthorized();
        }

        let token_type = *token_info.get(0).unwrap();
        if token_type != "Bearer" {
            return LoginUserData::unauthorized();
        }

        let token_content = token_info.get(1).unwrap().to_string();

        // get rocket config
        let config = request.guard::<&State<Config>>().await.succeeded().unwrap();

        return if let Ok(user_data) =
            verify_token::<Claims>(token_content, config.public_key.as_bytes())
        {
            Outcome::Success(LoginUserData {
                id: user_data.claims.id,
                username: user_data.claims.username,
                modes: user_data.claims.modes,
                verified_email: user_data.claims.verified_email,
            })
        } else {
            LoginUserData::unauthorized()
        };
    }
}
