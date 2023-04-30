use std::env;

use dotenv::dotenv;
use rocket::routes;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::views::add_roles::add_roles_view;
use crate::views::add_roles::okapi_add_operation_for_add_roles_view_;
use crate::views::check_access_token::check_access_token;
use crate::views::check_access_token::okapi_add_operation_for_check_access_token_;
use crate::views::check_role::check_role;
use crate::views::check_role::okapi_add_operation_for_check_role_;
use crate::views::index::index;
use crate::views::login::login;
use crate::views::login::okapi_add_operation_for_login_;

pub async fn rocket_main() -> Result<(), rocket::Error> {
    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URI not provided in .env");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_uri).await.expect("Pool was not created");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await.expect("Migrations failed");

    let _ = rocket::build()
        .manage::<PgPool>(pool)
        .mount("/", routes![index])
        .mount("/auth", openapi_get_routes![check_access_token, login, check_role, add_roles_view])
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
