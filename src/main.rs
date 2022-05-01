#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::Request;
use rocket_contrib::json::Json;

mod data;
mod util;

use data::response::Response;

#[catch(404)]
fn not_found(req: &Request) -> Json<Response> {
    Json(Response::not_found(req))
}

#[get("/teapot")]
fn teapot() -> Json<Response> {
    Json(Response::teapot(&Some("I'm a teapot!")))
}

#[get("/")]
fn index() -> Json<Response> {
    Json(Response::ok(&Some("hello world!")))
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/", routes![index, teapot])
        .launch();
}
