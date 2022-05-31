/// Authenticate APIs
mod authentication;
mod user;
mod verify_email;

use rocket::fairing::AdHoc;

#[doc(hidden)]
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load api stage", |rocket| async {
        rocket
            .attach(authentication::api::stage())
            .attach(verify_email::stage())
            .attach(user::stage())
    })
}
