use crate::Config;
use database::DB;
use rocket::{fairing::AdHoc, http::Status, serde::json::Json, State};
use util::jwt::{create_jwt_token, verify_token, Claims};

use crate::data::response::Response;

impl Response {
    fn debug_db(mut self, debug_db_names: Option<Vec<String>>) -> Self {
        self.code = Status::Ok.code;
        self.debug_db_names = debug_db_names;

        self
    }
}

#[get("/db")]
async fn debug_db(db: &State<DB>) -> Json<Response> {
    Response::default()
        .debug_db(
            db.client
                .as_ref()
                .unwrap()
                .list_database_names(None, None)
                .await
                .ok(),
        )
        .into()
}

#[get("/jwt")]
fn jwt_token(config: &State<Config>) -> String {
    create_jwt_token(config.private_key.as_bytes(), Claims { exp: 9999999999 }).unwrap()
}

#[get("/jwt/<token>")]
fn verify_jwt(token: &str, config: &State<Config>) -> String {
    verify_token(token.to_string(), config.public_key.as_bytes()).unwrap();

    "ok".to_string()
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("debug routes", |rocket| async {
        rocket.mount("/debug", routes![debug_db, jwt_token, verify_jwt])
    })
}
