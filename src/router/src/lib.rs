#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;

mod catch;
mod data;
mod db;
mod debug;
mod resource;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("catch stage", |rocket| async {
        rocket
            .attach(db::stage())
            .attach(catch::stage())
            .attach(resource::stage())
            .attach(debug::stage())
    })
}
