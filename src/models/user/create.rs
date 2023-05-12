use sqlx::{Error, PgPool};
use crate::password_utils;

pub async fn create_user(email: &str, password: &str, employee_id: i32, poll: &PgPool) -> Result<(), String> {
    let hashed_password = password_utils::hash_password(password);

    let result = sqlx::query!(r#"
        INSERT INTO "user" VALUES ($1, $2, $3)
    "#, email, hashed_password, employee_id)
        .execute(poll).await;
    return match result {
        Ok(_) => {
            Ok(())
        }
        Err(error) => match error {
            Error::Database(..) => Err("Введённый email уже существует".to_string()),
            error => Err(format!("Непредвиденная ошибка: {}", error))
        }
    };
}
