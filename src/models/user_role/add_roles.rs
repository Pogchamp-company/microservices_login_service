use sqlx::PgPool;
use crate::models::user_role::UserRole;

pub async fn add_roles(user_email: &str, roles: &[UserRole], pool: &PgPool) -> Result<(), String>{
    let roles = roles.to_vec();
    let result = sqlx::query!(r#"
        INSERT INTO user_to_role(user_email, role) SELECT $1, unnest($2::user_role[]) ON CONFLICT DO NOTHING;
    "#, user_email, roles as Vec<UserRole>).execute(pool)
        .await;

    match result {
        Ok(..) => Ok(()),
        Err(error) => Err(format!("Непредвиденная ошибка: {}", error))
    }
}
