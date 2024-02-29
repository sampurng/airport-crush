use crate::{collections::user::User, connection::DbConn};
use rocket::{http::Status, response::{status}, serde::json::Json, State};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct signup_response{
    id : String,
}

#[post("/signup", format = "json", data = "<user>")]
pub async fn signup(user: Json<User>, db_connection : &State<DbConn> ) -> status::Custom<Json<signup_response>> {
    format!("{:?}", user);
    let user: User = user.into_inner();
    
    match user.create_user(db_connection).await{
        Ok(res) => {
            let response: signup_response = signup_response{
                id : res.inserted_id.to_string(),
            };
            status::Custom(Status::Accepted, Json(response))
        }, 
        Err(e) => {
            let response = signup_response{
                id : e.kind.to_string()
            };
            status::Custom(Status::ExpectationFailed, Json(response))}
    }
}

// #[post("/authenticate", format="json", data="<claim>")]
// pub fn authenticate(claim : Claims){

// }
