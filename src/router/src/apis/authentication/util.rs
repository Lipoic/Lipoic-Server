use database::{
    Database,
    doc,
    model::auth::user::{ConnectAccount, User, UserMode},
    mongodb::bson,
};
use rocket::{response::status::BadRequest, serde::json::Json, State};
use util::{jwt::create_jwt_token, oauth::OAuthData, util::create_exp};

use crate::data::{
    code::Code,
    response::Response,
};

use super::data::{CreateUserInfo, RequestIp};
use database::mongodb::options::FindOneAndUpdateOptions;
use database::{Collection, Error};
use crate::apis::user::user_data::{Claims, Token};

pub async fn connect_account(
    oauth: OAuthData,
    code: String,
    db: &State<Database>,
    private_key: String,
    request_ip: RequestIp,
) -> Result<Json<Response<Token>>, BadRequest<Json<Response<String>>>> {
    let data = oauth
        .authorization_code(code)
        .await
        .map_err(|_| BadRequest(Some(Response::new(Code::OAuthCodeError, None))))?;

    let login_user_info = data
        .get_account_info(&oauth.account_type)
        .await
        .map_err(|_| BadRequest(Some(Response::new(Code::OAuthGetUserInfoError, None))))?;

    let user_data = create_and_update_user_info(
        db.user.as_ref().unwrap(),
        Some(ConnectAccount {
            account_type: oauth.account_type.clone(),
            name: login_user_info.name.clone(),
            email: login_user_info.email.clone(),
        }),
        vec![],
        None,
        CreateUserInfo {
            username: login_user_info.name.clone(),
            email: login_user_info.email.clone(),
            ip: request_ip.0,
            verified_email: login_user_info.verified_email,
        },
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
        private_key.as_bytes(),
        Claims {
            exp: create_exp(60 * 60 * 24 * 7),
            username: login_user_info.name,
            id: user_data._id.to_string(),
            verified_email: login_user_info.verified_email,
            modes: vec![UserMode::Student],
        },
    )
    .unwrap();

    // Response token.
    Ok(Response::new(Code::Ok, Some(Token { token })))
}

/// Update user info if it exists else insert
#[doc(hidden)]
pub async fn create_and_update_user_info(
    user: &Collection<User>,
    connect: Option<ConnectAccount>,
    modes: Vec<UserMode>,
    password_hash: Option<String>,
    user_info: CreateUserInfo,
) -> Result<Option<User>, Error> {
    let mut option = FindOneAndUpdateOptions::default();
    option.upsert = Some(true);

    // insert user info if not exists
    user.find_one_and_update(
        doc! { "email": &user_info.email },
        doc! {
            "$setOnInsert": {
                "username": &user_info.username,
                "email": &user_info.email,
                "verified_email": &user_info.verified_email,
                "modes": [],
                "login_ips": [],
                "password_hash": password_hash,
                "connects": []
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
                    "connects": bson::to_bson(&connect).unwrap()
                }
            },
            None,
        )
        .await?;
    }

    let user_data = user
        .find_one(doc! { "email": &user_info.email }, None)
        .await?;

    Ok(user_data)
}
