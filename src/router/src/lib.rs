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
            .attach(AdHoc::try_on_ignite("Token State", |rocket| async {
                match database::init().await {
                    Ok(client) => {
                        info!("Connected successfully.");
                        Ok(rocket.manage(database::DB { client }))
                    }
                    Err(_) => Err(rocket),
                }
            }))
            .attach(catch::stage())
            .attach(resource::stage())
    })
}
