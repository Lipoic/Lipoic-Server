use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::State;
use database::DB;
use crate::data::response::Response;

#[get("/")]
async fn hello_world(db: &State<DB>) -> Json<Response> {
    for database_name in db.client.list_database_names(None, None).await.unwrap() {
        println!("database name: {}", database_name);
    }

    Json(Response::ok(&Some("hello world!")))
}

#[get("/teapot")]
fn teapot() -> Json<Response> {
    Json(Response::teapot(&Some("I'm a teapot!")))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("catch stage", |rocket| async {
        rocket
            .mount("/", routes![hello_world, teapot])
    })
}