use std::env;

use sqlx::postgres::PgPoolOptions;

use crate::models::user::create_user;
use crate::models::user_role::{add_roles, UserRole};

pub async fn handle_create_director(mut args: Vec<String>) -> Result<(), String> {
    let admin_email = args.pop();
    let admin_password = args.pop();

    if let (Some(admin_email), Some(admin_password)) = (admin_email, admin_password) {
        let db_uri = env::var("DATABASE_URL").expect("DATABASE_URI not provided in .env");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_uri).await.expect("Pool was not created");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await.expect("Migrations failed");

        create_user(&admin_email, &admin_password, &pool).await?;
        add_roles(&admin_email, &[UserRole::Director], &pool).await;

        println!("Admin created successfully!");
        return Ok(());
    }

    return Err("Email and password are required".to_string());
}

pub fn print_help() {
    println!("Login service written in Rust");
    println!();
    println!("Commands:");
    println!("create_director <email> <password>");
    println!();
}

pub async fn handle_console_command() -> Result<bool, String> {
    let mut args: Vec<String> = env::args().rev().collect();

    let _ = args.pop();
    let query = args.pop();

    if let Some(query) = query {
        match query.as_str() {
            "create_director" => {
                handle_create_director(args).await?;
            },
            "help" | "--help" | "-h" => {
                print_help();
            }
            _ => {
                print_help();
                return Err(format!("Invalid command: {}", query));
            }
        }
        return Ok(true);
    }
    return Ok(false);
}
