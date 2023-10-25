use mongodb::{ bson::doc , Client, options::{ClientOptions, ServerApi, ServerApiVersion}};
use rocket::tokio;

#[tokio::main]
pub async fn connect_to_mongo()-> mongodb::error::Result<Client> {
    // Replace the placeholder with your Atlas connection string
    let uri ="mongodb://localhost:27017";
    let mut client_options = ClientOptions::parse(uri)?;
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;
    // Send a ping to confirm a successful connection
    client.database("admin").run_command(doc! { "ping": 1 }, None).await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    Ok(client)
}   

