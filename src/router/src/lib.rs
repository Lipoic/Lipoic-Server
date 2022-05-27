#[macro_use]
extern crate rocket;

use crate::db::db_init;
use rocket::fairing::AdHoc;
use rocket::serde::Deserialize;
use rocket::{Build, Rocket};

mod apis;
mod catch;
mod data;
mod db;
pub mod resource;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    private_key: String,
    public_key: String,
    mongodb_url: String,
    google_oauth_id: String,
    google_oauth_secret: String,
    google_account_email: String,
    google_account_password: String,
    issuer: String,
}

/// rocket server
pub async fn rocket(test: bool) -> Rocket<Build> {
    let rocket = rocket::build().attach(stage());
    let figment = rocket.figment();

    let config: Config = figment.extract().expect("config");

    if !test {
        db_init(rocket, config)
            .await
            .unwrap_or_else(|error| panic!("{:?}", error))
    } else {
        rocket.manage(database::DB {
            client: None,
            user: None,
        })
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load router stage", |rocket| async {
        rocket
            .attach(AdHoc::config::<Config>())
            .attach(catch::stage())
            .attach(resource::stage())
            .attach(apis::stage())
    })
}
