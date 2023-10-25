use serde::{Serialize, Deserialize};
use mongodb::Client;

#[derive(Debug, Serialize, Deserialize)]
struct User{
    email: &str, 
    password: &str, 
    user_name: Option<&str>,
    first_name: Option<&str>,
    last_name: Option<&str>,
    age : Option<u32>,
}

impl User{

    pub fn CreateUser(user: User, client : Client) -> Result<(), Error> {
        let user_collection = client.database("airport-crush").collection("Users");
        
        Ok(())
    }

}
