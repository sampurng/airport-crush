use crate::{collections::{user::User, claims}, connection::DbConn};
use rocket::{http::Status, response::status, serde::json::Json, State};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct SignupResponse{
    id : String,
    token: Option<String>,
}

#[post("/signup", format = "json", data = "<user>")]
pub async fn signup(user: Json<User>, db_connection : &State<DbConn> ) -> status::Custom<Json<SignupResponse>> {
    format!("{:?}", user);
    let user: User = user.into_inner();
    let user_jwt_token = claims::generate_token(&user).await;
    match user.create_user(db_connection).await{
        Ok(res) => {
            let response: SignupResponse = SignupResponse{
                id : res.inserted_id.to_string(),
                token: user_jwt_token
            };
            status::Custom(Status::Accepted, Json(response))
        }, 
        Err(e) => {
            let response = SignupResponse{
                id : e.kind.to_string(),
                token: None,
            };
            status::Custom(Status::ExpectationFailed, Json(response))}
    }
}

#[post("/saveDetails", format="json", data = "<user>")]
pub async fn save_details(user: Json<User>, db_connection : &State<DbConn>) -> status::Custom<Json<SignupResponse>>{
    format!("{:?}", user);
    let user = user.into_inner();
    match user.from_form(db_connection).await{
        Ok(res) => {
            let response: SignupResponse = SignupResponse{
                id : res.modified_count.to_string(),
                token: None,
            };
            status::Custom(Status::Accepted, Json(response))
        }, 
        Err(e) => {
            let response: SignupResponse = SignupResponse{
                id : e.kind.to_string(),
                token: None,
            };
            status::Custom(Status::ExpectationFailed, Json(response))
        }
    }
}