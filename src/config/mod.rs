use rocket::figment::providers::Env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub jwt_secret: String,
    pub jwt_iss: String,
    pub jwt_aud: String,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            jwt_secret: Env::var("JWT_SECRET").unwrap_or(String::from("")),
            jwt_iss: Env::var("JWT_ISS").unwrap_or(String::from("")),
            jwt_aud: Env::var("JWT_AUD").unwrap_or(String::from("")),
        }
    }
}

