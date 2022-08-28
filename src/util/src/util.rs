use std::time::{SystemTime, UNIX_EPOCH};
use database::mongodb::bson::oid::ObjectId;

// Expiration time
pub fn create_exp(time: usize) -> usize {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize)
        + time
}

/// Convert array of string to array of ObjectIds for MongoDB
pub fn string_vec_to_oid(list: Vec<String>) -> Vec<ObjectId> {
    list.iter().map(|string| {
        ObjectId::parse_str(string).unwrap()
    }).collect()
}