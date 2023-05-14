use rocket::http::Status;
use rocket::local::asynchronous::Client;
use sqlx::PgPool;
use login_service::models::user::create_user;
use login_service::models::user_role::{add_roles_by_email, UserRole};
use login_service::rocket_main::create_rocket;
use login_service::views::login::{LoginRequest, LoginResponse};
use crate::views::{TEST_USER_EMAIL, TEST_USER_EMPLOYEE_ID, TEST_USER_PASSWORD};

#[sqlx::test]
pub async fn test_login(pool: PgPool) -> Result<(), String> {

    create_user(TEST_USER_EMAIL, TEST_USER_PASSWORD,
                TEST_USER_EMPLOYEE_ID,
                &pool).await?;

    let roles = [UserRole::HumanResources];

    add_roles_by_email(TEST_USER_EMAIL, &roles, &pool).await?;

    let rocket = create_rocket(pool);

    let client = Client::tracked(rocket).await.unwrap();

    let req = client.post("/auth/login").json(&LoginRequest {
        email: TEST_USER_EMAIL.to_string(),
        password: TEST_USER_PASSWORD.to_string(),
    });

    let response = req.dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    let result: LoginResponse = response.into_json().await.expect("Json response expected");
    assert_eq!(result.roles, roles);

    Ok(())
}

#[sqlx::test]
pub async fn test_login_into_non_existent_user(pool: PgPool) {
    let rocket = create_rocket(pool);

    let client = Client::tracked(rocket).await.unwrap();

    let req = client.post("/auth/login").json(&LoginRequest {
        email: TEST_USER_EMAIL.to_string(),
        password: TEST_USER_PASSWORD.to_string(),
    });

    let response = req.dispatch().await;

    assert_eq!(response.status(), Status::Forbidden);
}
