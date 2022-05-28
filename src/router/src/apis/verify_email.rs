use crate::data::code::Code;
use crate::data::response::Response;
use crate::Config;
use database::{doc, DB};
use rocket::fairing::AdHoc;
use rocket::response::status::Unauthorized;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::State;
use util::email::VerifyEmailClaims;
use util::jwt::verify_token;

#[get("/verify-email?<code>")]
async fn verify_email<'a>(
    code: String,
    config: &'a State<Config>,
    db: &'a State<DB>,
) -> Result<Redirect, Unauthorized<Json<Response<'a, String>>>> {
    if let Ok(verify_user_data) =
        verify_token::<VerifyEmailClaims>(code, config.public_key.as_bytes())
    {
        db.user
            .as_ref()
            .unwrap()
            .find_one_and_update(
                doc! { "email": &verify_user_data.claims.email },
                doc! {
                    "$set": {
                        "verified_email": true
                    }
                },
                None,
            )
            .await
            .unwrap();

        Ok(Redirect::to("/"))
    } else {
        Err(Unauthorized(Some(Response::data(
            Code::VerifyEmailError,
            None,
        ))))
    }
}

#[doc(hidden)]
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load api stage", |rocket| async {
        rocket.mount("/", routes![verify_email])
    })
}
