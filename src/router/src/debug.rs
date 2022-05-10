use database::DB;
use rocket::{fairing::AdHoc, http::Status, serde::json::Json, State};

use crate::data::response::Response;

impl Response {
    fn db_debug(mut self, debug_db_names: Option<Vec<String>>) -> Self {
        self.code = Status::Ok.code;
        self.debug_db_names = debug_db_names;

        self
    }
}

#[get("/db/debug")]
async fn db_debug(db: &State<DB>) -> Json<Response> {
    Response::default()
        .db_debug(db.client.list_database_names(None, None).await.ok())
        .into()
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("debug routes", |rocket| async {
        rocket.mount("/", routes![db_debug])
    })
}
