use mongodb::{Client, results::InsertOneResult};
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
    age: Option<u32>,
    verified: bool,
}

impl User{
    pub async fn CreateUser(user: User, client : Client) -> Result<InsertOneResult, mongodb::error::Error> {
        let user_collection : Collection<User> = client.database("airport-crush").collection("Users");

        user_collection.insert_one(user, None).await
    }

    pub fn FromForm(user: User) -> Result<(), Error> {
        
        Ok(())
    }

}
