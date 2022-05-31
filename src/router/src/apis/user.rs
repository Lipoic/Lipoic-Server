use crate::data::auth_data::{AuthError, LoginUserData};
use crate::data::code::Code;
use crate::data::response::Response;
use crate::data::user::UserInfo;
use database::{doc, mongodb::bson::oid::ObjectId, Database};
use rocket::fairing::AdHoc;
use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;
use rocket::State;

/// # Get login user info
/// ## Request
/// - Path `/api/user/info`
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
/// curl -X GET -H "Authorization: Bearer {Token}" http://127.0.0.1:8000/api/user/info
/// ```
#[get("/info")]
async fn user_info(
    login_user_data: Result<LoginUserData, AuthError>,
    db: &State<Database>,
) -> Result<Json<Response<'static, UserInfo>>, AuthError> {
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
        Ok(Response::data(
            Code::Ok,
            Some(UserInfo {
                username: user_info.username,
                email: user_info.email,
                modes: user_info.modes,
                connects: user_info.connects,
            }),
        ))
    } else {
        Err(Unauthorized(Some(Response::data(
            Code::LoginUserNotFoundError,
            None,
        ))))
    }
}

#[doc(hidden)]
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load api stage", |rocket| async {
        rocket.mount("/api/user", routes![user_info])
    })
}
