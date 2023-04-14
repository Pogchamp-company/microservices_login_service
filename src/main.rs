#[macro_use]
extern crate rocket;

use std::env;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test/sqlx")]
async fn test_sqlx() -> String {
    let db_uri = env::var("DATABASE_URI").expect("DATABASE_URI not provided in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*db_uri).await.expect("Pool was not created");

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await.expect("Fetch failed");

    row.0.to_string()
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![test_sqlx])
}
