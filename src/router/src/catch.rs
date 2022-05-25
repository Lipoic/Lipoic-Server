use crate::data::error_code::ErrorCode;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::Request;

use crate::data::response::Response;

#[catch(404)]
fn not_found(req: &Request) -> Json<Response<String>> {
    Response::data(
        ErrorCode::NotFound,
        Some(format!("The requested page is invalid: {}", req.uri())),
        "Error".to_string(),
    )
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("catch stage", |rocket| async {
        rocket.register("/", catchers![not_found])
    })
}
