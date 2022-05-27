use crate::data::auth_data::{Auth, LoginFromData, SignUp, Token};
use crate::data::code::Code;
use crate::resource::Response;
use crate::Config;
use database::model::auth::user::{ConnectAccount, ConnectType, User, UserMode};
use database::mongodb::bson;
use database::mongodb::options::FindOneAndUpdateOptions;
use database::DB;
use database::{doc, Collection, Error};
use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::{BadRequest, Conflict};
use rocket::serde::json::Json;
use rocket::{Request, State};
use util::bcrypt::password_hash;
use util::jwt::{create_jwt_token, Claims};
use util::oauth::GoogleOAuth;
use util::util::create_exp;

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
fn google_oauth<'a>(redirect_uri: &'a str, config: &'a State<Config>) -> Json<Response<'a, Auth>> {
    let google_auth = GoogleOAuth::new(
        config.google_oauth_secret.clone(),
        config.google_oauth_id.clone(),
        config.issuer.clone(),
        redirect_uri,
    );

    Response::data(
        Code::Ok,
        Some(Auth {
            url: google_auth.get_auth_url(),
        }),
    )
}

/// Response data [Token]
///
/// # Parameters
/// * `code` - A OAuth2 code
/// * `oauth_redirect_uri` - A OAuth2 redirect uri
#[get("/google?<code>&<oauth_redirect_uri>")]
async fn google_oauth_code<'a>(
    code: String,
    oauth_redirect_uri: &'a str,
    config: &'a State<Config>,
    db: &'a State<DB>,
    request_ip: RequestIp,
) -> Result<Json<Response<'a, Token>>, BadRequest<Json<Response<'a, String>>>> {
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
                    None,
                ))));
            };

            let user_data = create_and_update_user_info(
                db.user.as_ref().unwrap(),
                login_user_info.name.clone(),
                login_user_info.email.clone(),
                login_user_info.verified_email.clone(),
                request_ip.0,
                Some(ConnectAccount {
                    account_type: ConnectType::Google,
                    name: login_user_info.name.clone(),
                    email: login_user_info.email.clone(),
                }),
                vec![],
                None,
            )
            .await
            .unwrap()
            .unwrap_or(
                db.user
                    .as_ref()
                    .unwrap()
                    .find_one(
                        doc! {
                            "email": &login_user_info.email
                        },
                        None,
                    )
                    .await
                    .unwrap()
                    .unwrap(),
            );

            let token = create_jwt_token(
                config.private_key.as_bytes(),
                Claims {
                    exp: create_exp(60 * 60 * 24 * 7),
                    email: login_user_info.email,
                    username: login_user_info.name,
                    id: user_data._id.to_string(),
                    verified_email: login_user_info.verified_email,
                },
            )
            .unwrap();

            // Response token.
            Ok(Response::data(Code::Ok, Some(Token { token })))
        }
        Err(_) => Err(BadRequest(Some(Response::data(
            Code::OAuthCodeError,
            None,
        )))),
    }
}

/// User login API
/// # Response
/// ## Response Code
/// * [Code::LoginUserNotFoundError]
/// * [Code::PasswordError] - Input password error.
/// ## Response Content
/// * [Token] - A JWT token.
#[post("/user/login", data = "<login_info>")]
async fn login<'a>(
    login_info: Form<LoginFromData>,
    db: &'a State<DB>,
    config: &'a State<Config>,
) -> Result<Json<Response<'a, Token>>, (Status, Json<Response<'a, String>>)> {
    let find_user = if let Some(user_data) = db
        .user
        .as_ref()
        .unwrap()
        .find_one(
            doc! {
                "email": &login_info.email,
            },
            None,
        )
        .await
        .unwrap()
    {
        user_data
    } else {
        // Response user not found.
        return Err((
            Status::Unauthorized,
            Response::data(
                Code::LoginUserNotFoundError,
                None,
            ),
        ));
    };

    if let Some(password_hash) = find_user.password_hash {
        // verify password correctness
        if util::bcrypt::verify_password(password_hash, login_info.password.clone()).unwrap() {
            let token = create_jwt_token(
                config.private_key.as_bytes(),
                Claims {
                    exp: create_exp(60 * 60 * 24 * 7),
                    email: find_user.email,
                    username: find_user.username,
                    id: find_user._id.to_string(),
                    verified_email: find_user.verified_email,
                },
            )
            .unwrap();

            // Response JWT.
            Ok(Response::data(Code::Ok, Some(Token { token })))
        } else {
            // Response input password error.
            Err((
                Status::Unauthorized,
                Response::data(
                    Code::LoginPasswordError,
                    None,
                ),
            ))
        }
    } else {
        // Response input password error.
        Err((
            Status::Unauthorized,
            Response::data(
                Code::LoginPasswordError,
                None,
            ),
        ))
    }
}

#[post("/user/sign-up", data = "<sign_up>")]
async fn sign_up<'a>(sign_up: Form<SignUp>, db: &'a State<DB>, config: &'a State<Config>, request_ip: RequestIp) -> Result<Json<Response<'a, String>>, Conflict<Json<Response<'a, String>>>> {
    let password_hash = password_hash(sign_up.password.clone()).unwrap();

    let user_data = create_and_update_user_info(
        db.user.as_ref().unwrap(),
        sign_up.username.clone(),
        sign_up.email.clone(),
        false,
        request_ip.0,
        None,
        sign_up.modes.0.clone(),
        Some(password_hash)
    ).await.unwrap();

    if user_data.is_none() {
        Ok(Response::data(
            Code::Ok,
            None
        ))
    } else {
        Err(Conflict(Some(
            Response::data(
                Code::SignUpEmailAlreadyRegistered,
                None,
            )
        )))
    }
}

/// Update user info if it exists else insert
async fn create_and_update_user_info(
    user: &Collection<User>,
    username: String,
    email: String,
    verified_email: bool,
    ip: String,
    connect: Option<ConnectAccount>,
    modes: Vec<UserMode>,
    password_hash: Option<String>
) -> Result<Option<User>, Error> {
    let mut option = FindOneAndUpdateOptions::default();
    option.upsert = Some(true);

    // insert user info if not exists
    let user_data = user
        .find_one_and_update(
            doc! { "email": &email },
            doc! {
                "$setOnInsert": {
                    "username": &username,
                    "email": &email,
                    "verified_email": verified_email,
                    "modes": ["Student"],
                    "login_ips": [],
                    "password_hash": password_hash,
                    "connect": []
                }
            },
            option,
        )
        .await?;

    // add login ip and modes
    user.update_one(
        doc! { "email": &email },
        doc! {
            "$addToSet": {
                "login_ips": ip,
                "modes": {
                    "$each": bson::to_bson(&modes).unwrap()
                },
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
        )
        .await?;
    }

    Ok(user_data)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load authentication stage", |rocket| async {
        rocket
            .mount(
                "/api/authentication",
                routes![google_oauth, google_oauth_code],
            )
            .mount("/api", routes![login, sign_up])
    })
}
