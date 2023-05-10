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

pub async fn delete_user(email: &str, pool: &PgPool) -> Result<(), String> {
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

#[derive(Default)]
pub struct LoadUserUserResult {
    pub email: String,
    pub password: String,
    pub roles: Vec<UserRole>
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
                roles: user_record.roles.unwrap()
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

#[cfg(test)]
mod test {
    use sqlx::PgPool;
    use crate::models::user::{create_user, delete_user, load_user, LoadUserUserResult};

    #[sqlx::test]
    pub async fn test_create_and_load_user(pool: PgPool) -> Result<(), String> {
        let email = "test@test.com";
        let password = "qwerty";
        let employee_id = 1;

        create_user(email, password, employee_id, &pool).await?;

        let user = load_user(email, &pool).await?;

        assert_eq!(user.email.as_str(), email);
        assert_eq!(user.roles, vec![]);

        Ok(())
    }

    #[sqlx::test]
    pub async fn test_create_and_delete_user(pool: PgPool) -> Result<(), String> {
        let email = "test@test.com";
        let password = "qwerty";
        let employee_id = 1;

        create_user(email, password, employee_id, &pool).await?;

        delete_user(email, &pool).await?;

        let user = load_user(email, &pool).await;

        match user {
            Ok(user) => {
                return Err(format!("User {} was not deleted", user.email))
            }
            Err(_) => {}
        }

        Ok(())
    }
}