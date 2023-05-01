use std::env;

use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicConsumeArguments, QueueBindArguments, QueueDeclareArguments};
use amqprs::connection::{Connection, OpenConnectionArguments};
use sqlx::postgres::PgPoolOptions;

use crate::consumers::RabbitMQConsumer;

pub async fn rabbit_main() -> Result<(), String> {
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

    Ok(())
}
