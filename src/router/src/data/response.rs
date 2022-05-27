use rocket::serde::json::Json;
use rocket::serde::ser::SerializeStruct;
use rocket::serde::Serialize;

use crate::data::code::Code;

#[derive(Debug)]
pub struct Response<'a, T> {
    pub code: Code<'a>,
    pub data: Option<T>,
}

impl<T: rocket::serde::Serialize> Serialize for Response<'_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        let mut state = serializer.serialize_struct("response", 3)?;

        state.serialize_field("code", &self.code.code)?;

        state.serialize_field("message", &self.code.message)?;

        self.data
            .as_ref()
            .and_then(|value| state.serialize_field("data", &value).ok());

        state.end()
    }
}

impl<T> Response<'_, T> {
    pub fn data(code: Code, data: Option<T>) -> Json<Response<T>> {
        Response {
            code,
            data,
        }
        .into()
    }
}
