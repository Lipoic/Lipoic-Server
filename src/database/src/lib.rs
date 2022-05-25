pub mod model;

pub use mongodb::error::Error;
use mongodb::{bson::doc, options::ClientOptions, Client};

pub struct DB {
    pub client: Option<Client>,
}

/// Init mongodb
pub async fn init(mongodb_url: String) -> mongodb::error::Result<DB> {
    let mut client_options = ClientOptions::parse(mongodb_url).await?;

    // Manually set an option
    client_options.app_name = Some("Lipoic Server".to_string());

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    let document = client
        .database("admin")
        .run_command(doc! {"ping": true}, None)
        .await?;
    print!("{}", document);

    Ok(DB {
        client: Some(client),
    })
}
