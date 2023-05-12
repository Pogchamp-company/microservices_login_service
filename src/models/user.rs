use sqlx::{Error, PgPool};

use crate::models::user_role::UserRole;
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

#[derive(Default)]
pub struct LoadUserUserResult {
    pub email: String,
    pub password: String,
    pub roles: Vec<UserRole>,
}

pub async fn load_user(email: &str, poll: &PgPool) -> Result<LoadUserUserResult, String> {
    let result = sqlx::query!(r#"
        SELECT
        email,
        password,
        array_remove(array_agg(role), NULL) as "roles: Vec<UserRole>"
        FROM "user"
        LEFT JOIN user_to_role
        ON user_to_role.user_email="user".email
        WHERE email = $1
        GROUP BY email
    "#, email).fetch_one(poll).await;

    match result {
        Ok(user_record) => {
            Ok(LoadUserUserResult {
                password: user_record.password,
                email: email.to_string(),
                roles: user_record.roles.unwrap(),
            })
        }
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                // todo Probably telling about that is bad idea
                Err("Пользователя с данной почтой не существует".to_string())
            }
            error => Err(format!("Непредвиденная ошибка: {}", error))
        }
    }
}

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
