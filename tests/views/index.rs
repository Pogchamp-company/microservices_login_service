use rocket::http::Status;
use rocket::local::asynchronous::Client;
use sqlx::PgPool;

use login_service::rocket_main::create_rocket;


#[sqlx::test]
pub async fn test_index(pool: PgPool) {
    let rocket = create_rocket(pool);

    let client = Client::tracked(rocket).await.unwrap();

    let req = client.get("/");

    let response = req.dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap().as_str(), "Hello, world!");
}
