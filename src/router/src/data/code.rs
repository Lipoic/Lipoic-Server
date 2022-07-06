use rocket::serde::Serialize;

/// # Example
/// ```rust
///generate_code! {
///    Ok(1, "Ok."),
///    NotFound(2, "Not found resource.")
///}
/// ```
macro_rules! generate_code {
    ($($name:ident ($code:expr, $message:literal)),+) => {
        $(
            #[doc=$message]
            #[allow(non_upper_case_globals)]
            pub const $name: Code = Code { code: $code, message: $message };
        )+
    }
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Code {
    pub(crate) code: usize,
    pub(crate) message: &'static str,
}

impl Code {
    generate_code! {
        Ok(200, "Ok."),
        NotFound(404, "Resource not found."),
        OAuthCodeError(1, "OAuth auth code error."),
        OAuthGetUserInfoError(2, "OAuth get user info error."),
        LoginUserNotFoundError(3, "User not found error."),
        LoginPasswordError(4, "Input password error."),
        SignUpEmailAlreadyRegistered(5, "This email is already registered."),
        VerifyEmailError(6, "This code is invalid."),
        AuthError(7, "This token is invalid."),
        EditUserFailed(8, "Edit the user info failed.")
    }
}
