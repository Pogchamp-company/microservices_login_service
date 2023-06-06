use sqlx::PgPool;
use crate::models::user_role::UserRole;


pub async fn user_exists(email: &str, poll: &PgPool) -> Result<bool, String> {
    let result = sqlx::query!(r#"
        SELECT count(email) as "exists!" FROM "user" WHERE email = $1
    "#, email).fetch_one(poll).await;

    match result {
        Ok(result) => {
            Ok(result.exists == 1)
        }
        Err(error) => Err(format!("Непредвиденная ошибка: {}", error))
    }
}
