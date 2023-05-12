use sqlx::PgPool;
use crate::models::user_role::UserRole;

pub async fn check_user_role(email: &str, role: &UserRole, poll: &PgPool) -> bool {
    let result = sqlx::query!(r#"
        SELECT EXISTS(
            SELECT user_email
            FROM user_to_role
            WHERE user_email = $1 AND role = $2
        ) AS "has_role: bool"
    "#, email, role as _).fetch_one(poll).await;

    return match result {
        Ok(record) => {
            record.has_role.unwrap()
        }
        Err(..) => {
            false
        }
    };
}
