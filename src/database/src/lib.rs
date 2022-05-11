extern crate dotenv;

use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;

pub struct DB {
    pub client: Client,
}

/// init mongodb
pub async fn init() -> mongodb::error::Result<DB> {
    // Load environment variables
    let mongodb_url = if let Ok(url) = env::var("MONGODB_URL") {
        // deploy mode
        url
    } else {
        // dev mode
        // load .env file
        dotenv::dotenv().expect("Failed to load .env file");
        env::var("MONGODB_URL").unwrap()
    };
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

    Ok(DB { client })
}
