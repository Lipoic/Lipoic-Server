use rocket::serde::Serialize;

/// # Example
/// ```
///generate_code! {
///    Ok => 1, "Ok.",
///    NotFound => 2, "Not found resource."
///}
/// ```
macro_rules! generate_code {
    ($($name:ident => $code:expr, $message:expr),+) => {
        $(
            #[doc=$message]
            #[allow(non_upper_case_globals)]
            pub const $name: Code<'static> = Code { code: $code, message: $message};
        )+
    }
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Code<'a> {
    pub(crate) code: usize,
    pub(crate) message: &'a str,
}

impl Code<'_> {
    generate_code! {
        Ok => 1, "Ok.",
        NotFound => 2, "Not found resource.",
        OAuthCodeError => 3, "OAuth auth code error.",
        OAuthGetUserInfoError => 4, "OAuth get user info error.",
        LoginUserNotFoundError => 5, "User not found error.",
        LoginPasswordError => 6, "Input password error.",
        SignUpEmailAlreadyRegistered => 7, "This email is already registered.",
        VerifyEmailError => 8, "This code is invalid."
    }
}
