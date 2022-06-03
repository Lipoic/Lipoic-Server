use crate::data::code::Code;
use crate::data::response::Response;
use crate::Config;
use database::{doc, Database};
use rocket::fairing::AdHoc;
use rocket::response::status::Unauthorized;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::State;
use util::email::VerifyEmailClaims;
use util::jwt::verify_token;

#[get("/verify-email?<code>")]
async fn verify_email(
    code: String,
    config: &State<Config>,
    db: &State<Database>,
) -> Result<Redirect, Unauthorized<Json<Response<String>>>> {
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
        Err(Unauthorized(Some(Response::new(
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
