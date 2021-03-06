use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::Request;

use crate::data::code::Code;
use crate::data::response::Response;

#[catch(404)]
fn not_found(_req: &Request) -> Json<Response<Option<String>>> {
    Response::new(Code::NotFound, None)
}

#[doc(hidden)]
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("catch stage", |rocket| async {
        rocket.register("/", catchers![not_found])
    })
}
