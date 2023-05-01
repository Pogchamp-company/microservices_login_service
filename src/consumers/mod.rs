use amqp_serde::types::{FieldName, FieldTable, FieldValue};
use amqprs::{BasicProperties, Deliver};
use amqprs::channel::Channel;
use amqprs::consumer::AsyncConsumer;
use rocket::serde::json::serde_json;
use sqlx::PgPool;

use crate::consumers::create_user::CreateUserSchema;
use crate::consumers::delete_user::DeleteUserSchema;

mod create_user;
mod delete_user;

pub struct RabbitMQConsumer {
    database_connection: PgPool,
}

impl RabbitMQConsumer {
    pub fn new(database_connection: PgPool) -> RabbitMQConsumer {
        return RabbitMQConsumer {
            database_connection
        };
    }
}

#[async_trait]
impl AsyncConsumer for RabbitMQConsumer {
    async fn consume(&mut self, _channel: &Channel, _deliver: Deliver, basic_properties: BasicProperties, content: Vec<u8>) {
        let raw_string = match std::str::from_utf8(&content) {
            Ok(raw_string) => raw_string,
            Err(..) => {
                println!("Could not parse byte content into raw string");
                return;
            }
        };

        let command_header_key: FieldName = "command".try_into().unwrap();
        let headers = match basic_properties.headers() {
            Some(headers) => headers,
            None => {
                println!("Headers was not provided");
                return;
            }
        };
        let command = match headers.get(&command_header_key) {
            Some(command) => command,
            None => {
                println!("'command' header was not provided");
                return;
            }
        };

        let command = match command {
            FieldValue::S(command) => command.to_string(),
            _ => {
                println!("'command' header must be a string");
                return;
            }
        };

        let result: Result<String, String> = match command.as_str() {
            "create_user" => {
                let json: CreateUserSchema = match serde_json::from_str(raw_string) {
                    Ok(raw_string) => raw_string,
                    Err(..) => {
                        println!("Could not parse raw string into json");
                        return;
                    }
                };

                create_user::consume(json, &self.database_connection).await
            },
            "delete_user" => {
                let json: DeleteUserSchema = match serde_json::from_str(raw_string) {
                    Ok(raw_string) => raw_string,
                    Err(..) => {
                        println!("Could not parse raw string into json");
                        return;
                    }
                };

                delete_user::consume(json, &self.database_connection).await
            },
            (unknown_command) => Err(format!("Unknown command: {}", unknown_command))
        };

        match result {
            Ok(success_message) => {
                println!("{}", success_message);
            }
            Err(error_message) => {
                eprintln!("{}", error_message);
            }
        }
    }
}

