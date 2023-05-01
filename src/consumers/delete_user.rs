use serde::Deserialize;
use sqlx::PgPool;

use crate::consumers::create_user::CreateUserSchema;
use crate::models::user::delete_user;

#[derive(Deserialize, Debug)]
pub struct DeleteUserSchema {
    email: String
}

pub async fn consume(request: DeleteUserSchema, database_connection: &PgPool) -> Result<String, String> {
    delete_user(&request.email, database_connection).await?;

    Ok(format!("User {} deleted successfully", request.email))
}