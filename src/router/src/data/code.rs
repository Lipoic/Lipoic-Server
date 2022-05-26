use self::Code::*;

#[derive(Debug)]
pub enum Code {
    Ok,
    /// not found resource
    NotFound,
    /// OAuth auth code error
    OAuthCodeError,
    /// OAuth get user info error
    OAuthGetUserInfoError,
    /// User not found error
    UserNotFound,
    /// Input password error
    PasswordError,
}

impl Code {
    pub fn get_code(&self) -> usize {
        match self {
            Ok => 1,
            NotFound => 2,
            OAuthCodeError => 3,
            OAuthGetUserInfoError => 4,
            UserNotFound => 5,
            PasswordError => 6,
        }
    }
}
