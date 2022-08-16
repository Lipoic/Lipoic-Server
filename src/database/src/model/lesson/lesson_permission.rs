use mongodb::bson::oid::ObjectId;

pub struct LessonPermission {
    pub permission_type: LessonPermissionType,
    pub allows: Option<Vec<ObjectId>>
}

pub enum LessonPermissionType {
    All,
    Classroom,
    Select,
}