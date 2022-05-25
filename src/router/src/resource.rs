use crate::data::error_code::Code;
pub use crate::data::response::Response;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

#[get("/")]
fn hello_world() -> Json<Response<String>> {
    Response::data(Code::Ok, None, String::from("hello world!"))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("catch stage", |rocket| async {
        rocket.mount("/", routes![hello_world])
    })
}
