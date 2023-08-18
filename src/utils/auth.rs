use std::collections::HashSet;
use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Deserialize, Serialize};
use crate::config::AppConfig;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iss: String,
    pub aud: String,
    pub iat: i64,
    pub jti: String,
    pub nbf: i64,
}

pub struct RefreshToken {
    pub token: String,
}

#[derive(Debug)]
pub struct UserInfo {
    pub id: String,
}

// Bearer Token
impl UserInfo {
    pub fn from_request(header: &str, app_config: AppConfig) -> Option<UserInfo> {
        let split_vec = header.split_whitespace().collect::<Vec<_>>();
        if split_vec.len() != 2 {
            return None;
        }
        if split_vec[0] != "Bearer" {
            return None;
        }
        Self::from_jwt(split_vec[1], app_config)
    }
    pub fn from_jwt(token_string: &str, app_config: AppConfig) -> Option<UserInfo> {
        let mut val = Validation::default();
        val.validate_nbf = true;
        val.validate_exp = true;
        val.iss = Some(HashSet::from([app_config.jwt_iss]));
        val.aud = Some(HashSet::from([app_config.jwt_aud]));
        match decode::<Claims>(token_string, &DecodingKey::from_secret(app_config.jwt_secret.as_bytes()), &val) {
            Ok(c) => {
                return Some(UserInfo {
                    id: c.claims.sub,
                });
            }
            Err(_) => None,
        }
    }
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserInfo {
    type Error = ();
    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let header_auth = request.headers().get_one("Authorization");
        let app_config = request.rocket().state::<AppConfig>().unwrap();
        if let Some(header_auth) = header_auth {
            if let Some(auth) = Self::from_request(header_auth, app_config.clone()) {
                return Outcome::Success(auth);
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RefreshToken {
    type Error = ();
    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let refresh_token = request.headers().get_one("RefreshToken");

        if let Some(refresh_token) = refresh_token {
            return Outcome::Success(RefreshToken {
                token: refresh_token.to_string(),
            });
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}