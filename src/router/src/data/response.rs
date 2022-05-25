use rocket::serde::json::Json;
use rocket::serde::ser::SerializeStruct;
use rocket::serde::Serialize;
use rocket::http::Status;

use crate::data::error_code::Code;

#[derive(Debug)]
pub struct Response<T> {
    pub code: Code,
    pub error_message: Option<String>,
    pub data: T,
}

impl<T: rocket::serde::Serialize> Serialize for Response<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        let mut state = serializer.serialize_struct("response", 3)?;

        state.serialize_field("code", &self.code.get_code())?;

        self.error_message
            .as_ref()
            .and_then(|v| state.serialize_field("error_message", &v).ok());

        state.serialize_field("data", &self.data)?;

        state.end()
    }
}

impl<T> Response<T> {
    pub fn data(
        error_code: Code,
        error_message: Option<String>,
        data: T,
    ) -> Json<Response<T>> {
        Response {
            error_message,
            code: error_code,
            data,
        }
        .into()
    }
}
