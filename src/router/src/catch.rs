use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{Request};

use crate::data::response::Response;

#[catch(404)]
fn not_found(req: &Request) -> Json<Response> {
    Json(Response::not_found(req))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("catch stage", |rocket| async {
        rocket
            .register("/", catchers![not_found])
    })
}