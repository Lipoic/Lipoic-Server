#[macro_use]
extern crate rocket;

use crate::db::db_init;
use rocket::fairing::AdHoc;
use rocket::serde::Deserialize;
use rocket::{Build, Rocket};

mod catch;
mod data;
mod db;
mod debug;
mod resource;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    test: bool,
    mongodb_url: String,
}

/// rocket server
pub async fn rocket() -> Rocket<Build> {
    let rocket = rocket::build().attach(stage());
    let figment = rocket.figment();

    let config: Config = figment.extract().expect("config");

    if config.test {
        db_init(rocket, config)
            .await
            .unwrap_or_else(|error| panic!("{:?}", error))
    } else {
        rocket
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load router stage", |rocket| async {
        rocket
            .attach(AdHoc::config::<Config>())
            .attach(catch::stage())
            .attach(resource::stage())
            .attach(debug::stage())
    })
}
