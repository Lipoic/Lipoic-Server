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

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    develop: bool,
    mongodb_url: String,
}

pub async fn rocket() -> Result<Rocket<Build>, rocket::Error> {
    let rocket = rocket::build().attach(stage());
    let figment = rocket.figment();

    let config: Config = figment.extract().expect("config");

    Ok(db_init(rocket, config)
        .await
        .unwrap_or_else(|error| panic!("{:?}", error)))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("catch stage", |rocket| async {
        rocket
            .attach(AdHoc::config::<Config>())
            .attach(catch::stage())
            .attach(resource::stage())
            .attach(debug::stage())
    })
}
