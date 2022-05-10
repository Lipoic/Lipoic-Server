#[macro_use]
extern crate rocket;

use database;
use rocket::fairing::AdHoc;

mod catch;
mod data;
mod resource;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("catch stage", |rocket| async {
        rocket
            .attach(AdHoc::try_on_ignite("Database State", |rocket| async {
                if let Ok(client) = database::init().await {
                    info!("Connected successfully.");
                    Ok(rocket.manage(database::DB { client }))
                } else {
                    Err(rocket)
                }
            }))
            .attach(catch::stage())
            .attach(resource::stage())
    })
}
