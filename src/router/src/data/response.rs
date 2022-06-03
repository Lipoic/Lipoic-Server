use rocket::serde::json::Json;
use rocket::serde::ser::SerializeStruct;
use rocket::serde::{Serialize, Serializer};

use crate::data::code::Code;

#[derive(Debug)]
pub struct Response<T> {
    pub code: Code,
    pub data: Option<T>,
}

impl<T: Serialize> Serialize for Response<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
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

impl<T> Response<T> {
    pub fn new(code: Code, data: Option<T>) -> Json<Response<T>> {
        Response { code, data }.into()
    }
}
