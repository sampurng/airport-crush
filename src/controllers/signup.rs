use crate::collections::user::User;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::serde::json::Json as JsonValue;
use serde_json;
use std::fmt::Error;

#[get("/world")]
pub fn index() {
    // let _dbs = connection::connect_to_mongo();
    // match dbs {
    //     Ok(res) => print!("{}", match res.get(0){
    //         Some(str) => str,
    //         _ => ""n
    //     }),
    //     Err(e) => print!("{}", e)
    // }
    ()
}

#[get("/databaseConnections")]
pub fn connections() -> &'static str {
    "hehe"
}

#[post("/lol", format = "json", data = "<user>")]
pub fn lol(user: Json<User>) -> String {
    // println!("{}", serde_json::to_string_pretty);
    format!("{:?}", user);
    let user: User = user.into_inner();
    serde_json::to_string(&user).unwrap()
    // "abs"
}
