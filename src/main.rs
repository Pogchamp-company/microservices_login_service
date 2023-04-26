#[macro_use]
extern crate rocket;

use std::env;

use dotenv::dotenv;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use views::add_user::add_user;
use crate::views::add_user::okapi_add_operation_for_add_user_;

use views::check_access_token::check_access_token;
use crate::views::check_access_token::okapi_add_operation_for_check_access_token_;

use views::check_role::check_role;
use crate::views::check_role::okapi_add_operation_for_check_role_;

use views::login::login;
use crate::views::login::okapi_add_operation_for_login_;

use views::add_role::add_roles_view;
use crate::views::add_role::okapi_add_operation_for_add_roles_view_;



mod password_utils;
mod models;
mod guards;
mod views;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
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
        .mount("/auth", openapi_get_routes![add_user, check_access_token, login, check_role, add_roles_view])
        .mount(
            "/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/auth/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .ignite().await?
        .launch().await?;

    Ok(())
}
