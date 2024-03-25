use rocket::{http::Status, Response};

#[get("/jwtError", format = "json")]
pub fn jwt_error() -> &'static str{
    "HELP"
}
