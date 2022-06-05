use crate::data::code::Code;
use crate::data::response::Response;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

#[get("/")]
fn hello_world() -> Json<Response<String>> {
    Response::new(Code::Ok, Some(String::from("hello world!")))
}

#[doc(hidden)]
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("catch stage", |rocket| async {
        rocket.mount("/", routes![hello_world])
    })
}
