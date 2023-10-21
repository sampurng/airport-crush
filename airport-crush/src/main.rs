#[macro_use] 
extern crate rocket;
mod connection;

#[get("/world")]
fn index(){
    let _dbs: Result<(), mongodb::error::Error> = connection::connect_to_mongo();
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
fn connections() -> &'static str {
    "hehe"
}

#[launch]
fn rocket() -> _ {
    let t = connection::connect_to_mongo();
    
    // let t: Result<(), mongodb::error::Error> = connect_to_mongo();
    rocket::build()
        .mount("/", routes![index, connections])
}