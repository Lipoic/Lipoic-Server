use rocket::fairing::AdHoc;
use rocket::fs::NamedFile;
use rocket::Request;

#[catch(404)]
async fn not_found(_: &Request<'_>) -> Option<NamedFile> {
    NamedFile::open("./resources/404.html").await.ok()
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("catch stage", |rocket| async {
        rocket.register("/", catchers![not_found])
    })
}
