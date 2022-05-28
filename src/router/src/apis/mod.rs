mod authentication;
mod verify_email;

use rocket::fairing::AdHoc;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load api stage", |rocket| async {
        rocket.attach(authentication::stage())
    })
}
