use std::env;

use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicConsumeArguments, QueueDeclareArguments};
use amqprs::connection::{Connection, OpenConnectionArguments};
use rocket::tokio::signal;

use crate::consumers::RabbitMQConsumer;
use crate::database_connection::create_connection_pool;

pub async fn rabbit_main() -> Result<(), String> {
    let addr = env::var("RABBITMQ_URI").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    let connection_arguments = OpenConnectionArguments::try_from(addr.as_str())
        .expect("Could not parse RABBITMQ_URI");

    let connection = Connection::open(&connection_arguments)
        .await
        .expect("Connection to RabbitMQ failed");
    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    let database_connection_pool = create_connection_pool();

    // open a channel on the connection
    let channel = connection.open_channel(None).await.unwrap();
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    let routing_key = "auth".to_string();

    // declare a queue
    let (queue_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::default().queue(routing_key).finish())
        .await
        .unwrap()
        .unwrap();

    //////////////////////////////////////////////////////////////////
    // start consumer with given name
    let args = BasicConsumeArguments::new(
        &queue_name,
        ""
    );

    channel
        .basic_consume(RabbitMQConsumer::new(database_connection_pool.await), args)
        .await
        .unwrap();

    return match signal::ctrl_c().await {
        Ok(()) => Ok(()),
        Err(err) => Err(format!("Failed to listen for ctrl+c because of {}", err))
    }
}
