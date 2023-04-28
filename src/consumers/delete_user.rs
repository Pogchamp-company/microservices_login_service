use serde::Deserialize;
use sqlx::PgPool;
use crate::consumers::create_user::CreateUserSchema;

#[derive(Deserialize, Debug)]
pub struct DeleteUserSchema {

}

pub async fn consume(request: DeleteUserSchema, database_connection: &PgPool) -> Result<(), String> {
    todo!("delete user")
}