use crate::data::response::Response;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

#[get("/")]
fn hello_world() -> Json<Response> {
    Response::default().ok(&Some("hello world!")).into()
}

#[get("/teapot")]
fn teapot() -> Json<Response> {
    Response::default().teapot(&Some("I'm a teapot!")).into()
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("catch stage", |rocket| async {
        rocket.mount("/", routes![hello_world, teapot])
    })
}
