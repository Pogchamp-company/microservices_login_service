use serde::Serialize;
use sqlx::PgPool;

#[derive(sqlx::Type, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRole {
    TaskManager,
    HumanResources,
    Director
}

pub async fn create_user(login: &str, password: &str, poll: &PgPool) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(r#"
        INSERT INTO "user" VALUES ($1, $2)
    "#, login, password)
        .execute(poll).await?;
    Ok(())
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