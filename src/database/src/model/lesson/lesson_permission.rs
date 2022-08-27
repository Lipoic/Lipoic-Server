use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use serde::Serialize;
use rocket::form::FromFormField;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LessonPermission {
    pub permission_type: LessonPermissionType,
    pub allows: Option<Vec<ObjectId>>
}

#[derive(Debug, Serialize, Deserialize, Clone, FromFormField)]
pub enum LessonPermissionType {
    /// All users can access this lesson.
    All,
    /// Only users in the classroom can access this lesson.
    Classroom,
    /// Only users in the allows list can access this lesson.
    Select,
}