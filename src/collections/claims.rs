

use mongodb::{bson::{DateTime}};
use std::time::{SystemTime, Duration};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Serialize, Deserialize};
use std::env;
use rocket::{fairing::{Fairing, Info, Kind}, http::{uri::Origin, Status}, Data, Request, Response};


use super::user::User;
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims{
    sub: String, 
    exp: DateTime,
}

pub struct JWT;

#[rocket::async_trait]
impl Fairing for JWT{
    fn info(&self) -> Info {
        Info { name:"Authenticating Request", kind: Kind::Request | Kind::Response }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        // Your JWT authentication logic here
        let jwt_string_recieved = match request.headers().get_one("Authorization"){
            Some(header) if header.starts_with("Bearer") => {
                header.trim_start_matches("Bearer ")
            }
            _ => {
                // jwt is not present, return from here with error
                request.set_uri(Origin::parse("/api/jwtError").unwrap());
                request.set_method(rocket::http::Method::Get);
                return;
            }
        };
        let key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY not found in environment variables");
        let validation = Validation {
            algorithms: vec![Algorithm::HS256], // Change the algorithm as needed
            ..Validation::default()
        };

        match decode::<Claims>(&jwt_string_recieved, &DecodingKey::from_secret(key.as_bytes()), &validation) {
            Ok(claim) => {
                if claim.claims.exp < DateTime::now(){
                    request.set_uri(Origin::parse("/jwtError").unwrap());
                    request.set_method(rocket::http::Method::Get);
                }
                return;
            },
            Err(_) => {
                request.set_uri(Origin::parse("/jwtError").unwrap());
                request.set_method(rocket::http::Method::Get);
                return; 
            }
        };
    }

}

pub async fn  generate_token(user:&User) -> Option<String>{
    let expiration_time = mongodb::bson::DateTime::from(SystemTime::now() + Duration::from_secs(2*60*60)) ; 
    let claims = Claims{
        sub: user.email.to_owned(),
        exp: expiration_time,
    };
    dotenv::dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY not found in environment variables");
    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    ) {
        Ok(token) => token,
        Err(_) => return None,      // Failed to generate JWT
    };
    Some(token)
}


