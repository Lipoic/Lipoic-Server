use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use serde::Serialize;
use rocket::form::FromForm;
use rocket::form::FromFormField;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LessonPermission {
    pub permission_type: LessonPermissionType,
    pub allows: Option<Vec<ObjectId>>
}

#[derive(Debug, Serialize, Deserialize, Clone, FromFormField)]
pub enum LessonPermissionType {
    All,
    Classroom,
    Select,
}