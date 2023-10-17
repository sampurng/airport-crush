#[macro_use] 
extern crate rocket;
use mongodb::{ bson::doc , Client, options::{ClientOptions, ServerApi, ServerApiVersion}};
use rocket::tokio;
// use rocket::serde::{Deserialize};



// #[derive(Deserialize)]
// #[serde(crate = "rocket::serde")]
// struct databaseList{
//     list : Vec<String>
// }

#[get("/world")]
fn index(){
    let _dbs: Result<(), mongodb::error::Error> = connect_to_mongo();
    // match dbs {
    //     Ok(res) => print!("{}", match res.get(0){
    //         Some(str) => str, 
    //         _ => ""
    //     }),
    //     Err(e) => print!("{}", e)
    // }
    ()
}

#[tokio::main]
async fn connect_to_mongo()-> mongodb::error::Result<()> {
    // Replace the placeholder with your Atlas connection string
    let uri ="mongodb://localhost:27017";
    let mut client_options = ClientOptions::parse(uri)?;
    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;
    // Send a ping to confirm a successful connection
    client.database("admin").run_command(doc! { "ping": 1 }, None);
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    Ok(())
}   

#[launch]
fn rocket() -> _ {
    let _ = connect_to_mongo();
    // let t: Result<(), mongodb::error::Error> = connect_to_mongo();
    rocket::build()
        .mount("/world", routes![index])
}