#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod test {
    use std::env;
    use rocket::futures::task::Spawn;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use sqlx::PgPool;
    use crate::database_connection::create_connection_pool;
    use crate::rocket_main::create_rocket;

    #[sqlx::test]
    pub async fn test_index(pool: PgPool) {
        let rocket = create_rocket(pool);

        let client = Client::tracked(rocket).await.unwrap();

        let req = client.get("/");

        let response = req.dispatch().await;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().await.unwrap().as_str(), "Hello, world!");
    }
}