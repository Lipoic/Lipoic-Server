use self::Code::*;

#[derive(Debug)]
pub enum Code {
    Ok,
    NotFound,
}

impl Code {
    pub fn get_code(&self) -> usize {
        match self {
            Ok => 200,
            NotFound => 404,
        }
    }
}
