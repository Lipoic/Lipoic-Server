use crate::apis::authentication::data::{CreateUserInfo, RequestIp};
use crate::apis::authentication::util::create_and_update_user_info;
use crate::data::auth_data::Claims;
use crate::data::auth_data::{AuthError, LoginUserData};
use crate::data::auth_data::{LoginFromData, SignUp, Token};
use crate::data::code::Code;
use crate::data::response::Response;
use crate::data::user::UserInfo;
use crate::Config;
use database::{doc, mongodb::bson::oid::ObjectId, Database};
use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status::{Conflict, Unauthorized};
use rocket::serde::json::Json;
use rocket::State;
use util::bcrypt::password_hash;
use util::email::{send_verify_email, VerifyEmailClaims};
use util::jwt::create_jwt_token;
use util::util::create_exp;

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
#[post("/login", data = "<login_info>")]
async fn login(
    login_info: Form<LoginFromData>,
    db: &State<Database>,
    config: &State<Config>,
) -> Result<Json<Response<Token>>, (Status, Json<Response<String>>)> {
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
            Response::new(Code::LoginUserNotFoundError, None),
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
            Ok(Response::new(Code::Ok, Some(Token { token })))
        } else {
            // Response input password error.
            Err((
                Status::Unauthorized,
                Response::new(Code::LoginPasswordError, None),
            ))
        }
    } else {
        // Response input password error.
        Err((
            Status::Unauthorized,
            Response::new(Code::LoginPasswordError, None),
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
#[post("/sign-up", data = "<sign_up>")]
async fn sign_up(
    sign_up: Form<SignUp>,
    db: &State<Database>,
    config: &State<Config>,
    request_ip: RequestIp,
) -> Result<Json<Response<String>>, Conflict<Json<Response<String>>>> {
    let password_hash = password_hash(&sign_up.password).unwrap();

    let user_data = create_and_update_user_info(
        db.user.as_ref().unwrap(),
        None,
        sign_up.modes.0.clone(),
        Some(password_hash),
        CreateUserInfo {
            username: sign_up.username.clone(),
            email: sign_up.email.clone(),
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
        Ok(Response::new(Code::Ok, None))
    } else {
        // Response email is already registered.
        Err(Conflict(Some(Response::new(
            Code::SignUpEmailAlreadyRegistered,
            None,
        ))))
    }
}

/// # Get login user info
/// ## Request
/// - Path `/user/info`
/// - [X] Authorization
/// ## Response
/// - Code
///     - [Code::Ok]
///     - [Code::AuthError]
///     - [Code::LoginUserNotFoundError]
/// - Content
///     - [UserInfo]
/// ## Curl Example
/// ```bash
/// curl -X GET -H "Authorization: Bearer {Token}" http://127.0.0.1:8000/user/info
/// ```
#[get("/info")]
async fn user_info(
    login_user_data: Result<LoginUserData, AuthError>,
    db: &State<Database>,
) -> Result<Json<Response<UserInfo>>, AuthError> {
    let login_user_data = match login_user_data {
        Ok(login_user_data) => login_user_data,
        Err(err) => return Err(err),
    };
    let find_user_data = db
        .user
        .as_ref()
        .unwrap()
        .find_one(
            doc! {
                "_id": ObjectId::parse_str(login_user_data.id).unwrap()
            },
            None,
        )
        .await
        .unwrap();

    if let Some(user_info) = find_user_data {
        Ok(Response::new(
            Code::Ok,
            Some(UserInfo {
                username: user_info.username,
                email: user_info.email,
                modes: user_info.modes,
                connects: user_info.connects,
            }),
        ))
    } else {
        Err(Unauthorized(Some(Response::new(
            Code::LoginUserNotFoundError,
            None,
        ))))
    }
}

#[doc(hidden)]
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load api stage", |rocket| async {
        rocket.mount("/user", routes![login, sign_up, user_info])
    })
}
