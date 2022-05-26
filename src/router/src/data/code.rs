use rocket::serde::Serialize;

/// # Example
/// ```
///generate_code! {
///    Ok => 1, "Ok.",
///    NotFound => 2, "Not found resource."
///}
/// ```
macro_rules! generate_code {
    ($($name:ident => $code:expr, $doc:expr),+) => {
        $(
            #[doc=$doc]
            #[allow(non_upper_case_globals)]
            pub const $name: Code = Code { code: $code };
        )+
    }
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Code {
    pub(crate) code: usize,
}

impl Code {
    generate_code! {
        Ok => 1, "Ok.",
        NotFound => 2, "Not found resource.",
        OAuthCodeError => 3, "OAuth auth code error.",
        OAuthGetUserInfoError => 4, "OAuth get user info error.",
        UserNotFound => 5, "User not found error.",
        PasswordError => 6, "Input password error."
    }
}
