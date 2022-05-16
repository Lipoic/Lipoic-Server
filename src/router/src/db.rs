use crate::Config;
use database::Error;
use rocket::{Build, Rocket};

pub async fn db_init(rocket: Rocket<Build>, config: Config) -> Result<Rocket<Build>, Error> {
    match database::init(config.mongodb_url).await {
        Ok(db) => {
            info!("Connected successfully.");
            Ok(rocket.manage(db))
        }
        Err(err) => Err(err),
    }
}
