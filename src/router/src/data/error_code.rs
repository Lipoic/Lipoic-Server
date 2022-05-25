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
}

impl Code {
    pub fn get_code(&self) -> usize {
        match self {
            Ok => 1,
            NotFound => 2,
            OAuthCodeError => 3,
            OAuthGetUserInfoError => 4,
        }
    }
}
