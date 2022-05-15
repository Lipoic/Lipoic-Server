use rocket::serde::ser::SerializeStruct;
use rocket::serde::Serialize;
use rocket::{http::Status, Request};

use util::util::get_string;

#[derive(Debug, Default)]
pub struct Response {
    pub code: u16,
    pub description: Option<String>,
    pub debug_db_names: Option<Vec<String>>,
}
impl Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        let mut state = serializer.serialize_struct("response", 3)?;

        state.serialize_field("code", &self.code)?;

        self.description
            .as_ref()
            .and_then(|v| state.serialize_field("description", &v).ok());

        self.debug_db_names
            .as_ref()
            .and_then(|v| state.serialize_field("debug_db_names", &v).ok());

        state.end()
    }
}

impl Response {
    pub fn ok(mut self, response: &Option<&'static str>) -> Self {
        self.code = Status::Ok.code;
        self.description = get_string(response);

        self
    }

    pub fn not_found(mut self, req: &Request) -> Self {
        self.code = Status::NotFound.code;
        self.description = Some(format!("The requested page is invalid: {}", req.uri()));

        self
    }
}
