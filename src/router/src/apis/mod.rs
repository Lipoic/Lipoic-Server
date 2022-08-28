/// Authenticate APIs
mod authentication;
mod user;
mod verify_email;
mod lesson;

use rocket::fairing::AdHoc;

#[doc(hidden)]
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load api stage", |rocket| async {
        rocket
            .attach(authentication::authentication_api::stage())
            .attach(verify_email::stage())
            .attach(user::user_api::stage())
            .attach(lesson::lesson_api::stage())
    })
}
