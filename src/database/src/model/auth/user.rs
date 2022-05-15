use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub integration: UserIntegration,
    pub identities: Vec<UserIdentity>,
    pub login_ips: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIntegration {
    pub google: String,
    pub facebook: String,
    pub taiwan_cloud_education: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserIdentity {
    Student,
    Teacher,
    Parents,
}
