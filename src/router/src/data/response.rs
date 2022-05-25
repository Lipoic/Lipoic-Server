use rocket::serde::json::Json;
use rocket::serde::ser::SerializeStruct;
use rocket::serde::Serialize;
use schemars::JsonSchema;

use crate::data::error_code::ErrorCode;

#[derive(Debug, JsonSchema)]
pub struct Response<T> {
    pub error_code: ErrorCode,
    pub error_message: Option<String>,
    pub data: T,
}

impl<T: rocket::serde::Serialize> Serialize for Response<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        let mut state = serializer.serialize_struct("response", 3)?;

        state.serialize_field("error_code", &self.error_code.get_error_code())?;

        self.error_message
            .as_ref()
            .and_then(|v| state.serialize_field("error_message", &v).ok());

        state.serialize_field("data", &self.data)?;

        state.end()
    }
}

impl<T> Response<T> {
    pub fn data(
        error_code: ErrorCode,
        error_message: Option<String>,
        data: T,
    ) -> Json<Response<T>> {
        Response {
            error_message,
            error_code,
            data,
        }
        .into()
    }
}
