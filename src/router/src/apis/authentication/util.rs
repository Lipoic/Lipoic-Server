use database::{
    doc,
    model::auth::user::{ConnectAccount, UserMode},
    Database,
};
use rocket::{response::status::BadRequest, serde::json::Json, State};
use util::{jwt::create_jwt_token, oauth::OAuthData, util::create_exp};

use crate::data::{
    auth_data::{Claims, Token},
    code::Code,
    response::Response,
};

use super::{
    api::create_and_update_user_info,
    data::{CreateUserInfo, RequestIp},
};

pub async fn connect_account<'a>(
    oauth: OAuthData<'_>,
    code: String,
    db: &'a State<Database>,
    private_key: String,
    request_ip: RequestIp,
) -> Result<Json<Response<'a, Token>>, BadRequest<Json<Response<'a, String>>>> {
    match oauth.authorization_code(code).await {
        Ok(data) => {
            let login_user_info = if let Ok(info) = data.get_account_info(oauth.account_type).await
            {
                info
            } else {
                return Err(BadRequest(Some(Response::data(
                    Code::OAuthGetUserInfoError,
                    None,
                ))));
            };

            let user_data = create_and_update_user_info(
                db.user.as_ref().unwrap(),
                Some(ConnectAccount {
                    account_type: oauth.account_type.clone(),
                    name: login_user_info.name.clone(),
                    email: login_user_info.email.clone(),
                }),
                &vec![],
                None,
                CreateUserInfo {
                    username: &login_user_info.name,
                    email: &login_user_info.email,
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
            Ok(Response::data(Code::Ok, Some(Token { token })))
        }
        Err(_) => Err(BadRequest(Some(Response::data(Code::OAuthCodeError, None)))),
    }
}
