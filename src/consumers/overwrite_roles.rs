use serde::Deserialize;
use sqlx::PgPool;

use crate::models::user_role::{add_roles_by_employee_id, delete_roles_by_employee_id, UserRole};

#[derive(Deserialize, Debug)]
pub struct OverwriteRolesSchema {
    employee_id: i32,
    roles: Vec<UserRole>,
}

pub async fn consume(request: OverwriteRolesSchema, database_connection: &PgPool) -> Result<String, String> {
    delete_roles_by_employee_id(request.employee_id, database_connection).await?;
    add_roles_by_employee_id(request.employee_id, &request.roles, database_connection).await?;

    Ok(format!("User with employee_id={} how has roles: {:?}", request.employee_id, request.roles))
}
