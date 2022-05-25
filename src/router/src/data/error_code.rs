use self::ErrorCode::*;

#[derive(Debug)]
pub enum ErrorCode {
    Ok,
    NotFound,
}

impl ErrorCode {
    pub fn get_error_code(&self) -> usize {
        match self {
            Ok => 200,
            NotFound => 404,
        }
    }
}
