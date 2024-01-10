use std::ptr::null;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};

#[macro_use] 
extern crate rocket;
mod connection;
mod controllers;
mod collections;

#[launch]
fn rocket() -> _ {
    let t = connection::connect_to_mongo();
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:19006/"]);
    
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-type", "Access-Control-Allow-Origin"]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors().unwrap();
    rocket::build()
        .mount("/api", routes![controllers::signup::index, controllers::signup::connections, controllers::signup::lol]).attach(cors)
}