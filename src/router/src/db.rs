use rocket::{fairing::AdHoc, Build, Rocket};

async fn attach_db(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    if let Ok(db) = database::init().await {
        info!("Connected successfully.");

        Ok(rocket.manage(db))
    } else {
        Err(rocket)
    }
}

pub fn stage() -> AdHoc {
    AdHoc::try_on_ignite("database state", attach_db)
}
