use sqlx::PgPool;

pub async fn delete_user_by_email(email: &str, pool: &PgPool) -> Result<(), String> {
    let result = sqlx::query!(r#"
        DELETE FROM "user" WHERE email = $1
    "#, email)
        .execute(pool).await;
    return match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return Err(format!("User {} already do not exist, so we can not delete them", email));
            }
            Ok(())
        },
        Err(error) => Err(error.to_string())
    }
}
