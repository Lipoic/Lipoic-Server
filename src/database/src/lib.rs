pub mod model;

pub use mongodb;
pub use mongodb::bson::doc;
pub use mongodb::error::Error;
pub use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};
use model::lesson::lesson_data::Lesson;
use crate::model::auth::user::User;

pub struct Database {
    pub client: Option<Client>,
    pub user: Option<Collection<User>>,
    pub lesson: Option<Collection<Lesson>>,
}

// Init mongodb
pub async fn init(mongodb_url: String) -> mongodb::error::Result<Database> {
    let mut client_options = ClientOptions::parse(mongodb_url).await?;

    // Manually set an option
    client_options.app_name = Some("Lipoic Server".to_string());

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    let db = client.database("lipoic_data");

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": true}, None)
        .await?;

    Ok(Database {
        client: Some(client),
        user: Some(db.collection::<User>("user")),
        lesson: Some(db.collection::<Lesson>("lesson")),
    })
}
