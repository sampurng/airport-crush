#[macro_use] extern crate rocket;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use rocket::tokio;


#[get("/world")]
fn index() -> &'static str {
    let dbs = connect_to_mongo();
    let sol = dbs.get(0).as_str();
    sol
}

#[tokio::main]
async fn connect_to_mongo() -> Result<Vec<String>, Box<dyn Error>>{
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())?;
    let client = Client::with_options(options)?;
    let mut databases : Vec<String> = Vec::new();

    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
       databases.push(name);
    }
    Ok(databases)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/world", routes![index])
        .mount("/hehehe", routes![index])
}