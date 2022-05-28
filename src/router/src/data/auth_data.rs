use rocket::http::Status;
use rocket::{Request, State};
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Unauthorized;
use database::model::auth::user::{UserMode};
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
use util::jwt::{verify_token};
use crate::Config;
use crate::data::code::Code;
use crate::data::response::Response;

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
    pub(crate) modes: Vec<UserMode>
}

#[doc(hidden)]
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginUserData {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) modes: Vec<UserMode>
}

pub type AuthError = Unauthorized<Json<Response<'static, String>>>;

#[rocket::async_trait]
#[doc(hidden)]
impl<'r> FromRequest<'r> for LoginUserData {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization = request.headers().get_one("Authorization");
        if let Some(token) = authorization {
            let token_info = token.split(' ').collect::<Vec<&str>>();
            let token_type = token_info.get(0);
            let token_content = token_info.get(1);
            if let Some(_token_type @ &"Bearer") = token_type {
                if let Some(token_content) = token_content {
                    println!("{}", token_content);
                    let config = request.guard::<&'r State<Config>>().await.succeeded().unwrap();
                    if let Ok(user_data) = verify_token::<Claims>(token_content.to_string(), config.public_key.as_bytes()) {
                        return Outcome::Success(LoginUserData {
                            id: user_data.claims.id,
                            username: user_data.claims.username,
                            modes: user_data.claims.modes
                        })
                    }
                }
            }
        }

        Outcome::Failure((Status::Unauthorized, Unauthorized(
            Some(Response::data(
                Code::AuthError,
                None,
            ))
        )))
    }
}