use sqlx::PgPool;

pub async fn delete_roles_by_employee_id(employee_id: i32, pool: &PgPool) -> Result<(), String>{
    let result = sqlx::query!(r#"
        DELETE FROM user_to_role WHERE user_email = (SELECT email FROM "user" WHERE employee_id = $1)
    "#, employee_id).execute(pool)
        .await;

    match result {
        Ok(..) => Ok(()),
        Err(error) => Err(format!("Непредвиденная ошибка: {}", error))
    }
}
