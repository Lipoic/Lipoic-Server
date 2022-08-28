#[macro_use]
extern crate rocket;

use crate::db::db_init;
use rocket::fairing::AdHoc;
use rocket::http::Method;
use rocket::serde::Deserialize;
use rocket::{Build, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};

mod apis;
mod catch;
pub mod data;
#[doc(hidden)]
mod db;
mod resource;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[doc(hidden)]
pub struct Config {
    private_key: String,
    public_key: String,
    mongodb_url: String,

    google_oauth_id: String,
    google_oauth_secret: String,
    google_account_email: String,
    google_account_password: String,

    facebook_oauth_id: String,
    facebook_oauth_secret: String,
    allowed_origins: Vec<String>,

    issuer: String,
}

/// rocket server
#[doc(hidden)]
pub async fn rocket(test: bool) -> Rocket<Build> {
    let rocket = rocket::build().attach(stage()).attach(cors_stage());

    let config: Config = rocket.figment().extract().expect("config");

    if !test {
        db_init(rocket, config)
            .await
            .unwrap_or_else(|error| panic!("{:?}", error))
    } else {
        rocket.manage(database::Database {
            client: None,
            user: None,
            lesson: None,
        })
    }
}

#[doc(hidden)]
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load router stage", |rocket| async {
        rocket
            .attach(AdHoc::config::<Config>())
            .attach(catch::stage())
            .attach(resource::stage())
            .attach(apis::stage())
    })
}

fn cors_stage() -> AdHoc {
    AdHoc::on_ignite("load CORS stage", |rocket| async {
        let config: Config = rocket.figment().extract().expect("config");

        let methods = vec![
            Method::Get,
            Method::Put,
            Method::Post,
            Method::Delete,
            Method::Options,
            Method::Patch,
        ]
            .into_iter()
            .map(From::from)
            .collect();

        let cors = CorsOptions::default()
            .allowed_origins(AllowedOrigins::some_exact(&config.allowed_origins))
            .allowed_methods(methods)
            .allow_credentials(true);

        rocket.attach(cors.to_cors().unwrap())
    })
}
