use bcrypt::{DEFAULT_COST, hash};
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::{State};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::json;
use validator::{Validate};
use crate::utils::id::create_uid;
use crate::utils::response::CustomResponse;
use super::super::models::user;
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::config::AppConfig;
use crate::models::user::Model;
use crate::utils::auth::{Claims, RefreshToken, UserInfo};

#[derive(Deserialize, Validate)]
pub struct SignupDTO {
    #[validate(length(min = 1, message = "username must not be empty"))]
    username: String,
    #[validate(length(min = 1, message = "password must not be empty"))]
    password: String,
    #[validate(length(min = 1, message = "email must not be empty"))]
    email: String,
}


#[derive(Deserialize, Validate)]
pub struct LoginDTO {
    #[validate(length(min = 1, message = "account must not be empty"))]
    account: String,
    #[validate(length(min = 1, message = "password must not be empty"))]
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginVO {
    access_token: String,
    refresh_token: String,
}


#[derive(Serialize, Deserialize)]
pub struct RefreshTokenVO {
    access_token: String,
}


#[post("/signup", data = "<signup_dto>")]
pub async fn signup(db: &State<DatabaseConnection>, app_config: &State<AppConfig>, signup_dto: Json<SignupDTO>) -> CustomResponse {
    if let Err(errors) = signup_dto.validate() {
        for (field, error_vec) in errors.field_errors().iter() {
            for error in &**error_vec {
                error!("{}:{}",field,error)
            }
        }
        return CustomResponse::Text(Status::BadRequest, String::from("bad request"));
    }

    let exit_user_result = user::Entity::find()
        .filter(user::Column::Username.eq(signup_dto.username.to_owned()).or(user::Column::Email.eq(signup_dto.email.to_owned())))
        .one(db.inner()).await;
    match exit_user_result {
        Ok(user) => {
            if user.is_some() {
                return CustomResponse::Text(Status::NotAcceptable, String::from("username or email is exit"));
            }
        }
        Err(err) => {
            error!("create_user error: {}", err);
            return CustomResponse::Text(Status::InternalServerError, String::from("signup failed"));
        }
    }

    let hashed_password = hash(signup_dto.password.to_owned(), DEFAULT_COST).unwrap_or(String::from(""));

    let id = create_uid();
    let user = user::ActiveModel {
        id: Set(id.to_owned()),
        username: Set(signup_dto.username.to_owned()),
        password: Set(hashed_password),
        email: Set(signup_dto.email.to_owned()),
        create_by: Set(id.to_owned()),
        update_by: Set(id.to_owned()),
        ..Default::default()
    };


    match user.insert(db.inner()).await {
        Ok(u) => {
            let access_token = gen_token(&app_config, u.id.to_owned(), chrono::Utc::now().checked_add_signed(chrono::Duration::minutes(5)).unwrap().timestamp());
            let refresh_token = gen_token(&app_config, u.id.to_owned(), chrono::Utc::now().checked_add_signed(chrono::Duration::days(30)).unwrap().timestamp());

            if access_token.is_empty() {
                return CustomResponse::Text(Status::InternalServerError, String::from("login failed"));
            }
            if refresh_token.is_empty() {
                return CustomResponse::Text(Status::InternalServerError, String::from("login failed"));
            }

            let vo = LoginVO {
                access_token,
                refresh_token,
            };
            return CustomResponse::Json(Status::Created, json!(vo));
        }
        Err(err) => {
            error!("create_user error: {}", err);
            return CustomResponse::Text(Status::InternalServerError, String::from("signup failed"));
        }
    }
}


#[post("/login", data = "<login_dto>")]
pub async fn login(db: &State<DatabaseConnection>, app_config: &State<AppConfig>, login_dto: Json<LoginDTO>) -> CustomResponse {
    let user_result = user::Entity::find()
        .filter(user::Column::Username.eq(login_dto.account.to_owned()).or(user::Column::Email.eq(login_dto.account.to_owned())))
        .one(db.inner()).await.unwrap_or(None);
    if user_result.is_none() {
        return CustomResponse::Text(Status::BadRequest, String::from("user not found"));
    }
    let user = user_result.unwrap();
    let is_match = bcrypt::verify(login_dto.password.to_owned(), user.password.to_owned().as_str()).unwrap_or(false);
    if !is_match {
        return CustomResponse::Text(Status::Unauthorized, String::from("password is invalid"));
    }

    let access_token = gen_token(&app_config, user.id.to_owned(), chrono::Utc::now().checked_add_signed(chrono::Duration::minutes(5)).unwrap().timestamp());
    let refresh_token = gen_token(&app_config, user.id.to_owned(), chrono::Utc::now().checked_add_signed(chrono::Duration::days(30)).unwrap().timestamp());

    if access_token.is_empty() {
        return CustomResponse::Text(Status::InternalServerError, String::from("login failed"));
    }
    if refresh_token.is_empty() {
        return CustomResponse::Text(Status::InternalServerError, String::from("login failed"));
    }

    let vo = LoginVO {
        access_token,
        refresh_token,
    };
    return CustomResponse::Json(Status::Created, json!(vo));
}


#[post("/refresh_token")]
pub async fn refresh_token(db: &State<DatabaseConnection>, app_config: &State<AppConfig>, user_info: UserInfo, refresh_token: RefreshToken) -> CustomResponse {
    let user_result = user::Entity::find()
        .filter(user::Column::Id.eq(user_info.id.to_owned()))
        .one(db.inner()).await.unwrap_or(None);
    if user_result.is_none() {
        return CustomResponse::Text(Status::Unauthorized, String::from("refresh_token is invalid"));
    }
    let refresh_user_info = UserInfo::from_jwt(refresh_token.token.as_str(), app_config.inner().clone());
    if refresh_user_info.is_none() {
        return CustomResponse::Text(Status::Unauthorized, String::from("refresh_token is invalid"));
    }

    let access_token = gen_token(&app_config, user_info.id.to_owned(), chrono::Utc::now().checked_add_signed(chrono::Duration::minutes(5)).unwrap().timestamp());

    if access_token.is_empty() {
        return CustomResponse::Text(Status::InternalServerError, String::from("login failed"));
    }

    let vo = RefreshTokenVO {
        access_token,
    };

    return CustomResponse::Json(Status::Created, json!(vo));
}


fn gen_token(app_config: &&State<AppConfig>, user_id: String, exp: i64) -> String {
    let access_token = encode(
        &Header::default(),
        &Claims {
            sub: user_id.to_owned(),
            exp,
            iss: String::from(app_config.jwt_iss.to_owned()),
            aud: String::from(app_config.jwt_aud.to_owned()),
            iat: chrono::Utc::now().timestamp(),
            jti: uuid::Uuid::new_v4().to_string(),
            nbf: chrono::Utc::now().timestamp(),
        }, &EncodingKey::from_secret(app_config.jwt_secret.as_bytes()),
    ).unwrap_or("".to_string());
    access_token
}
