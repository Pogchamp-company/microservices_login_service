#[macro_use]
extern crate rocket;

use std::env;

use dotenv::dotenv;
use rocket::response::status;
use rocket::serde::json::{Json, serde_json};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};
use sqlx::postgres::PgPoolOptions;

use models::user;
use password_utils::{create_jwt, get_email_from_token};
use views::check_access_token::check_access_token;
use views::register::register;

mod password_utils;
mod models;
mod views;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URI not provided in .env");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_uri).await.expect("Pool was not created");

    rocket::build()
        .manage::<PgPool>(pool)
        .mount("/", routes![index])
        .mount("/auth", routes![register, check_access_token])
}
