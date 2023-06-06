#[macro_use]
extern crate rocket;

pub mod views;
pub mod password_utils;
pub mod models;
pub mod guards;
pub mod consumers;

pub mod database_connection;

pub mod rabbitmq_main;
pub mod rocket_cors;
pub mod rocket_main;
pub mod cli_handler;
