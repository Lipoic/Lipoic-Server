/// Authenticate APIs
mod authentication;
mod verify_email;
mod user;

use rocket::fairing::AdHoc;

#[doc(hidden)]
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load api stage", |rocket| async {
        rocket
            .attach(authentication::stage())
            .attach(verify_email::stage())
            .attach(user::stage())
    })
}
