#[macro_use]
extern crate rocket;

use std::{env, str};
use amqp_serde::types::{FieldName, FieldTable, FieldValue};
use amqprs::{BasicProperties, Deliver};
use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicConsumeArguments, BasicPublishArguments, Channel, QueueBindArguments, QueueDeclareArguments};
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::consumer::{AsyncConsumer, DefaultConsumer};

use dotenv::dotenv;
use rocket::serde::json::serde_json;
use rocket::tokio;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use tokio::time;
use crate::consumers::RabbitMQConsumer;

pub mod views;
pub mod password_utils;
pub mod models;
pub mod guards;
pub mod consumers;

mod rocket_main;
mod rabbitmq_main;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let rocket_handle = rocket_main::rocket_main();
    let rabbit_handle = rabbitmq_main::rabbit_main();

    tokio::join!(rocket_handle, rabbit_handle);
}