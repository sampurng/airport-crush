
use collections::claims::{self, Claims};
use rocket_cors::{CorsOptions};

#[macro_use] 
extern crate rocket;
mod connection;
mod controllers;
mod collections;


#[launch]
async fn rocket()-> _ {
    // let allowed_origins = AllowedOrigins::some_exact(&["https://localhost:19006/"]);
    // let allowed_origins: rocket_cors::AllOrSome<rocket_cors::Origins> = AllowedOrigins::all();
    
    // let cors = rocket_cors::CorsOptions {
    //     allowed_origins,
    //     allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
    //     allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "content-type", "Access-Control-Allow-Origin", "X-Forwarded-For", "X-Forwarded-Proto", "Ngrok-Trace-Id"]),
    //     allow_credentials: true,
    //     ..Default::default()
    // }.to_cors().unwrap();

    let cors = CorsOptions::default().to_cors().unwrap();

    // let config = rocket::Config {
    //     address: "0.0.0.0".parse().unwrap(),
    //     ..rocket::Config::default()
    // };

    rocket::build().attach(claims::JWT)
        .mount("/api", routes![controllers::signup::signup, controllers::signup::save_details, controllers::helper::jwt_error])
        .manage(connection::DbConn::new().await)
        .attach(cors)
}