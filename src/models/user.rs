use rocket::execute;
use sqlx::{Executor, PgPool};
use sqlx::postgres::PgQueryResult;

pub async fn create_user(login: &String, password: &String, poll: &PgPool) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(r#"
        INSERT INTO "user" VALUES ($1, $2)
    "#, login, password)
        .execute(poll).await?;
    Ok(())
}