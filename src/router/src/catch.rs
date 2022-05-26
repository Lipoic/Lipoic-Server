use rocket::fairing::AdHoc;
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use rocket::Request;
use std::path::PathBuf;

#[catch(404)]
async fn not_found(_: &Request<'_>) -> NotFound<Option<NamedFile>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../resources/404.html");
    println!("{:?}", &path);
    NotFound(NamedFile::open(path).await.ok())
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("catch stage", |rocket| async {
        rocket.register("/", catchers![not_found])
    })
}
