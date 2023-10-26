#[macro_use] 
extern crate rocket;
mod connection;
mod controllers;

#[launch]
fn rocket() -> _ {
    let t = connection::connect_to_mongo();
     
    // let t: Result<(), mongodb::error::Error> = connect_to_mongo();
    rocket::build()
        .mount("/", routes![controllers::signup::index, controllers::signup::connections])
}