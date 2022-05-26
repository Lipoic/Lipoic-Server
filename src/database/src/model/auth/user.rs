use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub integration: Vec<UserIntegration>,
    pub modes: Vec<UserMode>,
    pub login_ips: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserIntegration {
    Google,
    Facebook,
    TaiwanCloudEducation,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserMode {
    Student,
    Teacher,
    Parents,
}
