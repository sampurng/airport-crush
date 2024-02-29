
use chrono::{DateTime, Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;

pub struct Claims{
    sub: String, 
    exp: DateTime<Utc>,
    user: User,
}

pub fn  generateToken(user:User) -> Option<String>{
    let expiration_time = Utc::now() + Duration::hours(2);
    let claims = Claims{
        sub: user.email.to_owned(),
        exp: expiration_time.timestamp()
    };
    dotenv::dotenv().ok();
    let jwtSercet = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY not found in environment variables");
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