use std::time::{SystemTime, UNIX_EPOCH};

// Expiration time
pub fn create_exp(time: usize) -> usize {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize)
        + time
}

pub fn get_redirect_uri_by_path(issuer: &str, path: &str) -> String {
    format!("{}{}", issuer, path)
}
