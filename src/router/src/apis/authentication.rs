use crate::data::auth_data::{Auth, Token};
use crate::data::error_code::Code;
use crate::resource::Response;
use crate::Config;
use database::model::auth::user::{ConnectAccount, ConnectType, User};
use database::mongodb::bson;
use database::mongodb::options::FindOneAndUpdateOptions;
use database::DB;
use database::{doc, Collection, Error};
use rocket::fairing::AdHoc;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::{Request, State};
use std::time::{SystemTime, UNIX_EPOCH};
use util::jwt::{create_jwt_token, Claims};
use util::oauth::GoogleOAuth;

/// Request Client IP Address
struct RequestIp(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestIp {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(RequestIp(request.client_ip().unwrap().to_string()))
    }
}

/// response google OAuth2 url
#[get("/google/url?<redirect_uri>")]
fn google_oauth(redirect_uri: &str, config: &State<Config>) -> Json<Response<Auth>> {
    let google_auth = GoogleOAuth::new(
        config.google_oauth_secret.clone(),
        config.google_oauth_id.clone(),
        config.issuer.clone(),
        redirect_uri,
    );

    Response::data(
        Code::Ok,
        None,
        Auth {
            url: google_auth.get_auth_url(),
        },
    )
}

/// Response data [Token]
///
/// # Parameters
/// * `code` - A OAuth2 code
/// * `oauth_redirect_uri` - A OAuth2 redirect uri
#[get("/google?<code>&<oauth_redirect_uri>")]
async fn google_oauth_code(
    code: String,
    oauth_redirect_uri: &str,
    config: &State<Config>,
    db: &State<DB>,
    request_ip: RequestIp,
) -> Result<Json<Response<Token>>, BadRequest<Json<Response<String>>>> {
    let google_auth = GoogleOAuth::new(
        config.google_oauth_secret.clone(),
        config.google_oauth_id.clone(),
        config.issuer.clone(),
        oauth_redirect_uri,
    );

    match google_auth.authorization_code(code).await {
        Ok(data) => {
            let login_user_info = if let Ok(info) = data.get_user_info().await {
                info
            } else {
                return Err(BadRequest(Some(Response::data(
                    Code::OAuthGetUserInfoError,
                    Some(String::from("Invalid OAuth token.")),
                    String::new(),
                ))));
            };
            // Expiration time: 1 weak
            let exp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs()
                + 60 * 60 * 24 * 7;

            let user_data = create_and_update_user_info(
                db.user.as_ref().unwrap(),
                login_user_info.name.clone(),
                login_user_info.email.clone(),
                request_ip.0,
                Some(ConnectAccount {
                    account_type: ConnectType::Google,
                    name: login_user_info.name.clone(),
                    email: login_user_info.email.clone(),
                }),
            )
            .await
            .unwrap();

            let token = create_jwt_token(
                config.private_key.as_bytes(),
                Claims {
                    exp: exp as usize,
                    email: login_user_info.email,
                    username: login_user_info.name,
                    id: user_data._id.to_hex(),
                },
            )
            .unwrap();

            Ok(Response::data(Code::Ok, None, Token { token }))
        }
        Err(_) => Err(BadRequest(Some(Response::data(
            Code::OAuthCodeError,
            Some(String::from("Invalid code.")),
            String::new(),
        )))),
    }
}

/// Update user info if it exists else insert
async fn create_and_update_user_info(
    user: &Collection<User>,
    username: String,
    email: String,
    ip: String,
    connect: Option<ConnectAccount>,
) -> Result<User, Error> {
    let mut option = FindOneAndUpdateOptions::default();
    option.upsert = Some(true);

    let user_data = user
        .find_one_and_update(
            doc! { "email": &email },
            doc! {
                "$setOnInsert": {
                    "username": &username,
                    "email": &email,
                    "modes": [],
                    "login_ips": [],
                    "password_hash": null,
                    "connect": []
                }
            },
            option,
        )
        .await?
        .unwrap_or(
            user.find_one(
                doc! {
                    "email": &email
                },
                None,
            )
            .await?
            .unwrap(),
        );

    user.update_one(
        doc! { "email": &email },
        doc! {
            "$addToSet": {
                "login_ips": ip,
                "modes": "Student",
            }
        },
        None,
    )
    .await?;

    if let Some(connect) = connect {
        user.update_one(
            doc! { "email": &email },
            doc! {
                "$addToSet": {
                    "connect": bson::to_bson(&connect).unwrap()
                }
            },
            None,
        ).await?;
    }

    Ok(user_data)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load authentication stage", |rocket| async {
        rocket.mount(
            "/api/authentication",
            routes![google_oauth, google_oauth_code],
        )
    })
}
