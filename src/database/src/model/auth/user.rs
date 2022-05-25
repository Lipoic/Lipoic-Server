use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub integration: UserIntegration,
    pub modes: Vec<UserMode>,
    pub login_ips: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIntegration {
    pub google: bool,
    pub facebook: bool,
    pub taiwan_cloud_education: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserMode {
    Student,
    Teacher,
    Parents,
}
