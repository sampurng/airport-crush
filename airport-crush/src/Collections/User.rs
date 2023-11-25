use mongodb::Client;
use serde::{Serialize, Deserialize};
use std::fmt::Error;
use mongodb::Collection; 
// use rocket::serde::Form;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User{
    email: String, 
    password: String, 
    user_name: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    age : Option<u32>,
}

// impl<'v> FromFormValue<'v> for User{

// } 

impl User{
    pub fn CreateUser(user: User, client : Client) -> Result<(), Error> {
        let user_collection : Collection<User> = client.database("airport-crush").collection("Users");
        
        Ok(())
    }

    pub fn FromForm(user: User) -> Result<(), Error> {

        Ok(())
    }

}
