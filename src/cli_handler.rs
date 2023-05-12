use std::env;

use sqlx::postgres::PgPoolOptions;

use crate::models::user::create_user;
use crate::models::user_role::{add_roles_by_email, UserRole};

pub async fn handle_create_director(mut args: Vec<String>) -> Result<String, String> {
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

        create_user(&admin_email, &admin_password, 0, &pool).await?;
        add_roles_by_email(&admin_email, &[UserRole::Director], &pool).await?;

        return Ok("Admin created successfully!".to_string());
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
        let result = match query.as_str() {
            "create_director" => {
                handle_create_director(args).await
            },
            "help" | "--help" | "-h" => {
                print_help();
                return Ok(true);
            }
            _ => {
                print_help();
                return Err(format!("Invalid command: {}", query));
            }
        };
        match result {
            Ok(success_message) => {
                println!("{}", success_message);
            }
            Err(error_message) => {
                eprintln!("{}", error_message);
            }
        }
        return Ok(true);
    }
    return Ok(false);
}
