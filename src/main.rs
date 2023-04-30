#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket::tokio;
use rocket_okapi::openapi_get_routes;
use sqlx::PgPool;

pub mod views;
pub mod password_utils;
pub mod models;
pub mod guards;
pub mod consumers;

mod rocket_main;
mod rabbitmq_main;
mod cli_handler;

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().ok();

    if cli_handler::handle_console_command().await? {
        return Ok(());
    }

    let rocket_handle = rocket_main::rocket_main();
    let rabbit_handle = rabbitmq_main::rabbit_main();

    let (rocket_result, rabbit_result) = tokio::join!(rocket_handle, rabbit_handle);

    rocket_result.map_err(|err| -> String {err.to_string()})?;
    rabbit_result?;

    Ok(())
}