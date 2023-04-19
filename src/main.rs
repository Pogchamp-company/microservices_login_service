#[macro_use]
extern crate rocket;

use std::env;

use dotenv::dotenv;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use views::check_access_token::check_access_token;
use views::login::login;
use views::register::register;
use views::check_role::check_role;

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

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await.expect("Migrations failed");

    rocket::build()
        .manage::<PgPool>(pool)
        .mount("/", routes![index])
        .mount("/auth", routes![register, check_access_token, login, check_role])
}
