use serde::Deserialize;
use sqlx::PgPool;

use crate::models::user::{delete_user_by_employee_id};

#[derive(Deserialize, Debug)]
pub struct DeleteUserSchema {
    employee_id: i32
}

pub async fn consume(request: DeleteUserSchema, database_connection: &PgPool) -> Result<String, String> {
    delete_user_by_employee_id(request.employee_id, database_connection).await?;

    Ok(format!("User with employee_id={} deleted successfully", request.employee_id))
}