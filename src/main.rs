#[macro_use] 
extern crate rocket;
mod connection;
mod controllers;
mod collections;

#[launch]
fn rocket() -> _ {
    let t = connection::connect_to_mongo();
    
    // let t: Result<(), mongodb::error::Error> = connect_to_mongo();
    rocket::build()
        .mount("/api", routes![controllers::signup::index, controllers::signup::connections, controllers::signup::lol])
}