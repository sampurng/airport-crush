use mongodb::{Client, Database};

pub struct DbConn {
    pub database: Database, // Adjust Document to match your data model
}

impl DbConn {
    pub async fn new() -> Self {
        let client = Client::with_uri_str("mongodb://localhost:27017").await.expect("Unable to reach DB");
        let db = client.database("admin");

        DbConn { database: db }
    }
    // Add other functions to interact with the database as needed
}

