use mongodb::{Client, results::InsertOneResult};
use rocket::State;
use serde::{Serialize, Deserialize};
use std::fmt::Error;
use mongodb::Collection;

use crate::connection::DbConn; 
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
    pub async fn create_user(&self, client : &State<DbConn>) -> Result<InsertOneResult, mongodb::error::Error> {
        let user_collection : &Collection<User> = &client.database.collection("test");
        user_collection.insert_one(self, None).await
    }

    pub fn from_form(user: User) -> Result<(), Error> {
        
        Ok(())
    }

}
