use mongodb::{bson::{self, bson, doc, to_bson}, results::{InsertOneResult, UpdateResult}, Client};
use rocket::State;
use serde::{Serialize, Deserialize};
use mongodb::Collection;

use crate::connection::DbConn; 
// use rocket::serde::Form;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User{
    pub email: String, 
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

    pub async fn from_form(&self, db_connection: &State<DbConn>) -> Result<UpdateResult, mongodb::error::Error> {
        let user_collection: &Collection<User> = &db_connection.database.collection("test");
        let filter = doc! {"email": &self.email};
        let user_bson = mongodb::bson::to_bson(self).unwrap();
        let update = doc! {"$set" : user_bson};
        user_collection.update_one(filter, update, None).await
    }

}
