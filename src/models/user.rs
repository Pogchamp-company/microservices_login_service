use rocket::execute;
use sqlx::{Error, Executor, PgPool};
use sqlx::postgres::PgQueryResult;

pub async fn create_user(login: &String, password: &String, poll: &PgPool) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(r#"
        INSERT INTO "user" VALUES ($1, $2)
    "#, login, password)
        .execute(poll).await?;
    Ok(())
}

pub struct LoadUserUserResult {
    pub email: String,
    pub password: String
}

pub async fn load_user(email: &str, poll: &PgPool) -> Result<LoadUserUserResult, String> {
    let result = sqlx::query!(r#"
        SELECT password FROM "user" WHERE email = $1
    "#, email).fetch_one(poll).await;

    match result {
        Ok(user_record) => {
            Ok(LoadUserUserResult {
                password: user_record.password,
                email: email.to_string()
            })
        }
        Err(err) => match err {
            Error::RowNotFound => {
                // todo Probably telling about that is bad idea
                Err("Пользователя с данной почтой не существует".to_string())
            }
            error => Err(format!("Непредвиденная ошибка: {}", error))
        }
    }
}