use rocket::serde::json::Json;
use rocket::serde::ser::SerializeStruct;
use rocket::serde::Serialize;

use crate::data::code::Code;

#[derive(Debug)]
pub struct Response<T> {
    pub code: Code,
    pub error_message: Option<String>,
    pub data: Option<T>,
}

impl<T: rocket::serde::Serialize> Serialize for Response<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        let mut state = serializer.serialize_struct("response", 3)?;

        state.serialize_field("code", &self.code)?;

        self.error_message
            .as_ref()
            .and_then(|v| state.serialize_field("error_message", &v).ok());
        self.data
            .as_ref()
            .and_then(|value| state.serialize_field("data", &value).ok());

        state.end()
    }
}

impl<T> Response<T> {
    pub fn data(code: Code, error_message: Option<String>, data: Option<T>) -> Json<Response<T>> {
        Response {
            error_message,
            code,
            data,
        }
        .into()
    }
}
