use rocket::serde::Serialize;
use rocket::{http::Status, Request};

use util::util::get_string;

#[derive(Debug, Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub code: u16,
    pub description: Option<String>,
    pub debug_db_names: Option<Vec<String>>,
}

impl Response {
    pub fn ok(mut self, response: &Option<&'static str>) -> Self {
        self.code = Status::Ok.code;
        self.description = get_string(response);

        self
    }

    pub fn teapot(mut self, response: &Option<&'static str>) -> Self {
        self.code = Status::ImATeapot.code;
        self.description = get_string(response);

        self
    }

    pub fn not_found(mut self, req: &Request) -> Self {
        self.code = Status::NotFound.code;
        self.description = Some(format!("The requested page is invalid: {}", req.uri()));

        self
    }
}
