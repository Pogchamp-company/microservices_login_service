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

use views::add_user::add_user;
use crate::views::add_user::okapi_add_operation_for_add_user_;

use views::check_access_token::check_access_token;
use crate::views::check_access_token::okapi_add_operation_for_check_access_token_;

use views::check_role::check_role;
use crate::views::check_role::okapi_add_operation_for_check_role_;

use views::login::login;
use crate::views::login::okapi_add_operation_for_login_;

use views::add_roles::add_roles_view;
use crate::views::add_roles::okapi_add_operation_for_add_roles_view_;

use tokio::time;
use crate::consumers::RabbitMQConsumer;

mod password_utils;
mod models;
mod guards;
mod views;
mod consumers;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let addr = env::var("RABBITMQ_URI").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    let connection_arguments = OpenConnectionArguments::try_from(addr.as_str())
        .expect("Could not parse RABBITMQ_URI");

    let connection = Connection::open(&connection_arguments)
        .await
        .unwrap();
    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URI not provided in .env");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_uri).await.expect("Pool was not created");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await.expect("Migrations failed");

    // open a channel on the connection
    let channel = connection.open_channel(None).await.unwrap();
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    // declare a queue
    let (queue_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::default())
        .await
        .unwrap()
        .unwrap();

    // bind the queue to exchange
    let rounting_key = "amqprs.example";
    let exchange_name = "amq.topic";
    channel
        .queue_bind(QueueBindArguments::new(
            &queue_name,
            exchange_name,
            rounting_key,
        ))
        .await
        .unwrap();

    //////////////////////////////////////////////////////////////////
    // start consumer with given name
    let args = BasicConsumeArguments::new(
        &queue_name,
        "example_basic_pub_sub"
    );

    channel
        .basic_consume(RabbitMQConsumer::new(pool), args)
        .await
        .unwrap();

    //////////////////////////////////////////////////////////////////
    // publish message
    let content = String::from(
        r#"
        {
            "email": "example@example.com",
            "password": "qwerty",
            "roles": ["human_resources"],
            "employee_id": 1
        }
    "#,
    )
        .into_bytes();

    // create arguments for basic_publish
    let args = BasicPublishArguments::new(exchange_name, rounting_key);

    let mut headers = FieldTable::new();
    headers.insert(FieldName::try_from("command").unwrap(), FieldValue::from("create_user"));
    let properties = BasicProperties::default()
        .with_headers(headers).finish();

    channel
        .basic_publish(properties, content, args)
        .await
        .unwrap();


    // channel/connection will be closed when drop.
    // keep the `channel` and `connection` object from dropping
    // before pub/sub is done.
    time::sleep(time::Duration::from_secs(1)).await;
    // explicitly close
    channel.close().await.unwrap();
    connection.close().await.unwrap();
}
