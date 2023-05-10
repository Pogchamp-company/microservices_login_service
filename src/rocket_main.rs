use std::env;

use rocket::{Build, Rocket, routes};
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use sqlx::PgPool;
use crate::database_connection::create_connection_pool;

use crate::views::add_roles::add_roles_view;
use crate::views::add_roles::okapi_add_operation_for_add_roles_view_;
use crate::views::check_access_token::check_access_token;
use crate::views::check_access_token::okapi_add_operation_for_check_access_token_;
use crate::views::check_role::check_role;
use crate::views::check_role::okapi_add_operation_for_check_role_;
use crate::views::index::index;
use crate::views::login::login;
use crate::views::login::okapi_add_operation_for_login_;

pub fn create_rocket(database_connection_pool: PgPool) -> Rocket<Build> {
    rocket::build()
        .manage::<PgPool>(database_connection_pool)
        .mount("/", routes![index])
        .mount("/auth", openapi_get_routes![check_access_token, login, check_role, add_roles_view])
        .mount(
            "/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/auth/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}

pub async fn rocket_main() -> Result<(), rocket::Error> {
    let pool: PgPool = create_connection_pool().await;

    let _ = create_rocket(pool)
        .ignite().await?
        .launch().await?;

    Ok(())
}
