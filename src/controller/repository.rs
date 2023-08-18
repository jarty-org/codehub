use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use crate::utils::auth::UserInfo;

#[derive(Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Internal,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRepositoryDTO {
    name: String,
    description: String,
    visibility: Visibility,
    initialize_with_readme: bool,
    gitignore_template: Option<String>,
    license_template: Option<String>,
}

#[post("/repository", data = "<create_repository_dto>")]
pub fn create_repository(user_info: UserInfo, create_repository_dto: Json<CreateRepositoryDTO>) -> String {
    return "ok".to_string();
}