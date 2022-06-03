use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

#[doc(hidden)]
pub struct CreateUserInfo {
    pub username: String,
    pub email: String,
    pub verified_email: bool,
    pub ip: String,
}

/// Request Client IP Address
#[doc(hidden)]
pub struct RequestIp(pub String);

#[rocket::async_trait]
#[doc(hidden)]
impl<'r> FromRequest<'r> for RequestIp {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(RequestIp(request.client_ip().unwrap().to_string()))
    }
}
