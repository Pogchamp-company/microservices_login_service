use serde::Deserialize;
use sqlx::PgPool;

use crate::consumers::RabbitMQConsumer;
use crate::models::user::create_user;
use crate::models::user_role::UserRole;

#[derive(Deserialize, Debug)]
pub struct CreateUserSchema {
    email: String,
    password: String,
    roles: Vec<UserRole>,
    employee_id: i32,
}

pub async fn consume(request: CreateUserSchema, database_connection: &PgPool) -> Result<(), String> {
    create_user(&request.email, &request.password, database_connection).await?;
    Ok(())
}