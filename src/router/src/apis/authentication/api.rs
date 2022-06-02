use database::model::auth::user::{ConnectAccount, ConnectType, User, UserMode};
use database::mongodb::bson;
use database::mongodb::options::FindOneAndUpdateOptions;
use database::Database;
use database::{doc, Collection, Error};
use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status::{BadRequest, Conflict};
use rocket::serde::json::Json;
use rocket::State;
use util::bcrypt::password_hash;
use util::email::{send_verify_email, VerifyEmailClaims};
use util::jwt::create_jwt_token;
use util::oauth::OAuthData;
use util::util::create_exp;

use crate::data::auth_data::Claims;
use crate::data::auth_data::{AuthUrl, LoginFromData, SignUp, Token};
use crate::data::code::Code;
use crate::data::response::Response;
use crate::Config;

use super::data::{CreateUserInfo, RequestIp};
use super::util::connect_account;

/// # Get Google OAuth url
/// ## Request
/// - Path `/authentication/google/url`
/// - Parameters
///     - `redirect_uri`
/// ## Response
/// - Code
///     - [Code::Ok]
/// - Content
///     - [AuthUrl] - A OAuth url
#[get("/google/url?<redirect_uri>")]
fn google_oauth<'a>(
    redirect_uri: &'a str,
    config: &'a State<Config>,
) -> Json<Response<'a, AuthUrl>> {
    let google_auth = OAuthData::new(
        &ConnectType::Google,
        &config.google_oauth_secret,
        &config.google_oauth_id,
        redirect_uri,
    );

    Response::data(
        Code::Ok,
        Some(AuthUrl {
            url: google_auth.get_auth_url(),
        }),
    )
}

/// # Google OAuth2 login
/// ## Request
/// - Path `/authentication/google`
/// - Parameters
///     - `code` - A OAuth2 code
///     - `oauth_redirect_uri` - A OAuth2 redirect uri
/// ## Response
/// - Response Code
///     - [Code::Ok]
///     - [Code::OAuthCodeError]
/// - Response Content
///     - [Token] - A login token.
/// ## Curl Example
/// ```bash
/// curl -X GET http://127.0.0.1:8000/authentication/google?code={code}&oauth_redirect_uri={oauth_redirect_uri}
/// ```
#[get("/google?<code>&<oauth_redirect_uri>")]
async fn google_oauth_code<'a>(
    code: String,
    oauth_redirect_uri: &'a str,
    config: &'a State<Config>,
    db: &'a State<Database>,
    request_ip: RequestIp,
) -> Result<Json<Response<'a, Token>>, BadRequest<Json<Response<'a, String>>>> {
    let google_auth = OAuthData::new(
        &ConnectType::Google,
        &config.google_oauth_secret,
        &config.google_oauth_id,
        oauth_redirect_uri,
    );

    return connect_account(
        google_auth,
        code,
        db,
        config.private_key.clone(),
        request_ip,
    )
    .await;
}

/// # Get Facebook OAuth url
/// ## Request
/// - Path `/authentication/facebook/url`
/// - Parameters
///     - `redirect_uri`
/// ## Response
/// - Code
///     - [Code::Ok]
/// - Content
///     - [AuthUrl] - A OAuth url
#[get("/facebook/url?<redirect_uri>")]
fn facebook_oauth<'a>(
    redirect_uri: &'a str,
    config: &'a State<Config>,
) -> Json<Response<'a, AuthUrl>> {
    let facebook_auth = OAuthData::new(
        &ConnectType::Facebook,
        &config.facebook_oauth_secret,
        &config.facebook_oauth_id,
        redirect_uri,
    );

    Response::data(
        Code::Ok,
        Some(AuthUrl {
            url: facebook_auth.get_auth_url(),
        }),
    )
}

/// # Facebook OAuth2 login
/// ## Request
/// - Path `/authentication/facebook`
/// - Parameters
///     - `code` - A OAuth2 code
///     - `oauth_redirect_uri` - A OAuth2 redirect uri
/// ## Response
/// - Response Code
///     - [Code::Ok]
///     - [Code::OAuthCodeError]
/// - Response Content
///     - [Token] - A login token.
/// ## Curl Example
/// ```bash
/// curl -X GET http://127.0.0.1:8000/authentication/facebook?code={code}&oauth_redirect_uri={oauth_redirect_uri}
/// ```
#[get("/facebook?<code>&<oauth_redirect_uri>")]
async fn facebook_oauth_code<'a>(
    code: String,
    oauth_redirect_uri: &'a str,
    config: &'a State<Config>,
    db: &'a State<Database>,
    request_ip: RequestIp,
) -> Result<Json<Response<'a, Token>>, BadRequest<Json<Response<'a, String>>>> {
    let facebook_auth = OAuthData::new(
        &ConnectType::Facebook,
        &config.facebook_oauth_secret,
        &config.facebook_oauth_id,
        oauth_redirect_uri,
    );

    return connect_account(
        facebook_auth,
        code,
        db,
        config.private_key.clone(),
        request_ip,
    )
    .await;
}

/// # User login API
/// ## Request
/// - Path `/user/login`
/// - FromData [LoginFromData]
/// ## Response
/// - Code
///     - [Code::LoginUserNotFoundError]
///     - [Code::LoginPasswordError] - Input password error.
///     - [Code::Ok]
/// - Content
///     - [Token] - A JWT token.
/// ## Curl Example
/// ```bash
/// curl -X POST -F email=aijdfajodwsdf@gmail.com -F password=123 http://127.0.0.1:8000/user/login
/// ```
#[post("/user/login", data = "<login_info>")]
async fn login<'a>(
    login_info: Form<LoginFromData>,
    db: &'a State<Database>,
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
            Response::data(Code::LoginUserNotFoundError, None),
        ));
    };

    if let Some(password_hash) = find_user.password_hash {
        // verify password correctness
        if util::bcrypt::verify_password(password_hash, &login_info.password).unwrap() {
            let token = create_jwt_token(
                config.private_key.as_bytes(),
                Claims {
                    exp: create_exp(60 * 60 * 24 * 7),
                    username: find_user.username,
                    id: find_user._id.to_string(),
                    verified_email: find_user.verified_email,
                    modes: find_user.modes,
                },
            )
            .unwrap();

            // Response JWT.
            Ok(Response::data(Code::Ok, Some(Token { token })))
        } else {
            // Response input password error.
            Err((
                Status::Unauthorized,
                Response::data(Code::LoginPasswordError, None),
            ))
        }
    } else {
        // Response input password error.
        Err((
            Status::Unauthorized,
            Response::data(Code::LoginPasswordError, None),
        ))
    }
}

/// # Sign up account API
/// ## Request
/// - Path `/user/sign-up`
/// - FromData [SignUp]
/// ## Response
/// - Code
///     - [Code::SignUpEmailAlreadyRegistered]
///     - [Code::Ok]
/// - Content
///     - [Code::Ok]
/// ## Curl Example
/// ```bash
/// curl -X POST -F email=aijdfajodwsdf@gmail.com -F password=123 -F username=abc -F modes='["Student"]' http://127.0.0.1:8000/user/sign-up
/// ```
#[post("/user/sign-up", data = "<sign_up>")]
async fn sign_up<'a>(
    sign_up: Form<SignUp>,
    db: &'a State<Database>,
    config: &'a State<Config>,
    request_ip: RequestIp,
) -> Result<Json<Response<'a, String>>, Conflict<Json<Response<'a, String>>>> {
    let password_hash = password_hash(&sign_up.password).unwrap();

    let user_data = create_and_update_user_info(
        db.user.as_ref().unwrap(),
        None,
        &sign_up.modes.0,
        Some(password_hash),
        CreateUserInfo {
            username: &sign_up.username,
            email: &sign_up.email,
            ip: request_ip.0,
            verified_email: false,
        },
    )
    .await
    .unwrap();

    if user_data.is_none() {
        // The verify email code.
        let code = create_jwt_token(
            config.private_key.as_bytes(),
            VerifyEmailClaims {
                exp: create_exp(60 * 10),
                email: sign_up.email.clone(),
            },
        )
        .unwrap();

        send_verify_email(
            &config.google_account_email,
            &config.google_account_password,
            &config.issuer,
            String::from("/verify-email"),
            code,
            &sign_up.email,
        );

        // Response Ok.
        Ok(Response::data(Code::Ok, None))
    } else {
        // Response email is already registered.
        Err(Conflict(Some(Response::data(
            Code::SignUpEmailAlreadyRegistered,
            None,
        ))))
    }
}

/// Update user info if it exists else insert
#[doc(hidden)]
pub async fn create_and_update_user_info(
    user: &Collection<User>,
    connect: Option<ConnectAccount>,
    modes: &Vec<UserMode>,
    password_hash: Option<String>,
    user_info: CreateUserInfo<'_>,
) -> Result<Option<User>, Error> {
    let mut option = FindOneAndUpdateOptions::default();
    option.upsert = Some(true);

    // insert user info if not exists
    let user_data = user
        .find_one_and_update(
            doc! { "email": &user_info.email },
            doc! {
                "$setOnInsert": {
                    "username": &user_info.username,
                    "email": &user_info.email,
                    "verified_email": &user_info.verified_email,
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
        doc! { "email": &user_info.email },
        doc! {
            "$addToSet": {
                "login_ips": &user_info.ip,
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
            doc! { "email": &user_info.email },
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

#[doc(hidden)]
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load authentication stage", |rocket| async {
        rocket
            .mount(
                "/authentication",
                routes![
                    google_oauth,
                    google_oauth_code,
                    facebook_oauth,
                    facebook_oauth_code
                ],
            )
            .mount("/", routes![login, sign_up])
    })
}
