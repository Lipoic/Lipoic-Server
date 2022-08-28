use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::State;
use database::{Database, doc};
use database::model::lesson::lesson_data::Lesson;
use database::model::lesson::lesson_permission::{LessonPermission, LessonPermissionType};
use database::model::lesson::lesson_state::LessonState;
use database::mongodb::bson::DateTime;
use database::mongodb::bson::oid::ObjectId;
use util::util::string_vec_to_oid;
use crate::apis::lesson::lesson_api_data::LessonApiData;
use crate::apis::user::user_data::{AuthError, LoginUserData};
use crate::data::code::Code;
use crate::data::response::Response;

/// # Create a lesson
/// ## Request
/// - Path `/lesson`
/// - Method `POST`
/// - FromData [LessonApiData]
/// - [X] Authorization
/// ## Response
/// - Code
///     - [Code::AuthError]
///     - [Code::Ok]
/// - Content
///     - [Lesson]
#[post("/", data = "<lesson_data>")]
async fn create_lesson(
    login_user_data: Result<LoginUserData, AuthError>,
    db: &State<Database>,
    lesson_data: Form<LessonApiData>,
) -> Result<Json<Response<Lesson>>, AuthError> {
    // Check the user is logged in.
    let login_user_data = match login_user_data {
        Ok(login_user_data) => login_user_data,
        Err(err) => return Err(err),
    };

    let classroom_id: Option<ObjectId>;
    if let Some(_classroom_id) = lesson_data.classroom_id.clone() {
        classroom_id = Some(ObjectId::parse_str(&_classroom_id).unwrap());
    } else {
        classroom_id = None;
    }

    let permission_allows: Option<Vec<ObjectId>>;
    if let Some(_permission_allows) = lesson_data.permission.allows.clone() {
        permission_allows = Some(string_vec_to_oid(_permission_allows))
    } else {
        permission_allows = None;
    }


    let lesson = Lesson {
        _id: ObjectId::new(),
        name: lesson_data.name.clone(),
        description: lesson_data.description.clone(),
        created_at: DateTime::now(),
        create_by: login_user_data.id.parse().unwrap(),
        speakers: string_vec_to_oid(lesson_data.speakers.clone()),
        state: LessonState::Draft,
        permission: LessonPermission {
            permission_type: lesson_data.permission.permission_type.clone(),
            allows: permission_allows,
        },
        classroom_id,
    };

    db.lesson.as_ref().unwrap().insert_one(lesson.clone(), None).await.unwrap();

    Ok(Response::new(Code::Ok, Some(lesson.clone())))
}

/// # Get lesson info
/// ## Request
/// - Path `/lesson`
/// - Method `GET`
/// - FromData [LessonApiData]
/// - [X] Authorization
/// ## Response
/// - Code
///     - [Code::Ok]
///     - [Code::Forbidden]
///     - [Code::NotFound]
/// - Content
///     - [Lesson]
#[get("/<id>")]
async fn get_lesson(id: String, login_user_data: Result<LoginUserData, AuthError>, db: &State<Database>) -> Json<Response<Lesson>> {
    let lesson = db.lesson.as_ref().unwrap().find_one(doc! {
        "_id": ObjectId::parse_str(&id).unwrap()
    }, None).await.unwrap();

    if let Some(lesson) = lesson {
        let permission = &lesson.permission;
        let permission_type = &permission.permission_type;
        let mut can_access = false;

        match permission_type {
            LessonPermissionType::All => {
                can_access = true;
            }
            LessonPermissionType::Classroom => {
                // TODO: Check if the user is in the classroom.
                can_access = true;
            }
            LessonPermissionType::Select => {
                let allow_users = &permission.allows;

                if login_user_data.is_ok() && allow_users.is_some() {
                    let user_id = &login_user_data.unwrap().id.parse().unwrap();
                    can_access = allow_users.as_ref().unwrap().contains(user_id);
                }
            }
        }

        if can_access {
            Response::new(Code::Ok, Some(lesson))
        } else {
            Response::new(Code::Forbidden, None)
        }
    } else {
        Response::new(Code::NotFound, None)
    }
}

/// # Edit lesson info
#[patch("/")]
async fn edit_lesson() -> Json<Response<String>> {
    // TODO: Edit lesson info.
    Response::new(Code::Ok, Some(String::from("TODO")))
}

#[doc(hidden)]
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("load lesson api stage", |rocket| async {
        rocket.mount(
            "/lesson",
            routes![create_lesson, get_lesson, edit_lesson],
        )
    })
}
