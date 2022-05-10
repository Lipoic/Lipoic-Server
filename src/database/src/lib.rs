extern crate dotenv;

use std::env;
use mongodb::{bson::doc, options::ClientOptions, Client};

pub struct DB {
    pub client: Client
}

/// init mongodb
pub async fn init() -> mongodb::error::Result<Client> {
    // Load environment variables
    dotenv::dotenv().expect("Failed to load .env file");

    let mongodb_url = env::var("MONGODB_URL").unwrap();
    let mut client_options = ClientOptions::parse(mongodb_url).await?;

    // Manually set an option
    client_options.app_name = Some("Lipoic Server".to_string());

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    if let Err(err) = client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await
    {
        panic!("Failed to connect to the database: {:#?}", err);
    }

    Ok(client)
}