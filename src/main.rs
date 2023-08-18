mod controller;
mod db;
mod models;
mod utils;
mod config;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate validator_derive;
extern crate validator;


use rocket::fairing::AdHoc;
use rocket::figment::providers::Env;
use controller::user::{signup, login, refresh_token};
use controller::system::not_found;
use controller::repository::create_repository;
use crate::db::create_db;

#[launch]
async fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("address", Env::var("ADDRESS").unwrap_or(String::from("0.0.0.0"))))
        .merge(("port", Env::var("PORT").unwrap_or(String::from("3000")).parse::<u16>().unwrap_or(3000)));


    rocket::custom(figment)
        .attach(AdHoc::try_on_ignite("App Config", |rocket| async {
            let app_config = config::AppConfig::new();
            Ok(rocket.manage(app_config))
        }))
        .attach(AdHoc::try_on_ignite("Database Setup", |rocket| async {
            let db_url = Env::var("DATABASE_URL").unwrap_or("".parse().unwrap());
            if db_url.is_empty() {
                error!("DATABASE_URL is not set");
                return Err(rocket);
            }
            let db = create_db(db_url).await.unwrap_or_else(|err| {
                error!("create_db error: {}", err);
                panic!("fail to connect db or create db pool");
            });
            Ok(rocket.manage(db))
        })).mount("/api/v1", routes![signup,login,refresh_token,create_repository])
        .mount("/", routes![not_found])
}