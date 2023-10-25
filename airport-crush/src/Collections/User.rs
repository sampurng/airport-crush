#[derive(Serialize, Deserialize)]
struct User{
    email: &str, 
    password: &str, 
    user_name: Option<&str>,
    first_name: Option<&str>,
    last_name: Option<&str>,
    age : u32,
}