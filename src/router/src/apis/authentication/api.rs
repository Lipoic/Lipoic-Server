use database::model::auth::user::ConnectType;

use database::doc;
use database::Database;
use rocket::fairing::AdHoc;

use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use util::oauth::OAuthData;

use crate::data::auth_data::{AuthUrl, Token};
use crate::data::code::Code;
use crate::data::response::Response;
use crate::Config;

use super::data::RequestIp;
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

#[doc(hidden)]
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load authentication stage", |rocket| async {
        rocket.mount(
            "/authentication",
            routes![
                google_oauth,
                google_oauth_code,
                facebook_oauth,
                facebook_oauth_code
            ],
        )
    })
}
