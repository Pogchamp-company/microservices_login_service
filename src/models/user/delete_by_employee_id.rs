use sqlx::PgPool;

pub async fn delete_user_by_employee_id(employee_id: i32, pool: &PgPool) -> Result<(), String> {
    let result = sqlx::query!(r#"
        DELETE FROM "user" WHERE employee_id = $1
    "#, employee_id)
        .execute(pool).await;
    return match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return Err(format!("User with employee_id={} already do not exist, so we can not delete them", employee_id));
            }
            Ok(())
        }
        Err(error) => Err(error.to_string())
    };
}
