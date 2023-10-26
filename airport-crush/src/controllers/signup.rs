use core::fmt::Error;
// use std::collections;

#[get("/world")]
pub fn index(){
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

#[post("/signup", format = "appication/json", data = "<user>")]
pub fn signup(user: collections::user::User) -> Result<(), Error> {
    println!("{}", user);
    Ok(())
}




