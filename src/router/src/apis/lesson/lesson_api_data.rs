use database::model::lesson::lesson_permission::LessonPermissionType;

#[derive(FromForm)]
pub struct LessonApiData {
    pub name: String,
    pub description: Option<String>,
    pub speakers: Vec<String>,
    pub permission: LessonPermission,

    pub classroom_id: Option<String>,
}


#[derive(FromForm, Clone)]
pub struct LessonPermission {
    pub permission_type: LessonPermissionType,
    pub allows: Option<Vec<String>>,
}