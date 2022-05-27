use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub email: String,
    pub verified_email: bool,
    pub password_hash: Option<String>,
    pub connect: Vec<ConnectAccount>,
    pub modes: Vec<UserMode>,
    pub login_ips: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConnectType {
    Google,
    Facebook,
    TaiwanCloudEducation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectAccount {
    pub account_type: ConnectType,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserMode {
    Student,
    Teacher,
    Parents,
}
