use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use crate::model::lesson::lesson_permission::LessonPermission;
use crate::model::lesson::lesson_state::LessonState;
use serde::Deserialize;
use serde::Serialize;

/// https://hackmd.io/@lipoic/S1k6xgio5
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lesson {
    pub _id: ObjectId,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime,
    pub create_by: ObjectId,
    pub speakers: Vec<ObjectId>,
    pub state: LessonState,
    pub permission: LessonPermission,

    pub classroom_id: Option<ObjectId>,
}