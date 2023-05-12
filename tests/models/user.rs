use sqlx::PgPool;
use login_service::models::user::{create_user, delete_user_by_email, delete_user_by_employee_id, load_user};

static TEST_USER_EMAIL: &str = "test@test.com";
static TEST_USER_PASSWORD: &str = "qwerty";
static TEST_USER_EMPLOYEE_ID: i32 = 1;

#[sqlx::test]
pub async fn test_create_and_load_user(pool: PgPool) -> Result<(), String> {
    create_user(TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_EMPLOYEE_ID, &pool).await?;

    let user = load_user(TEST_USER_EMAIL, &pool).await?;

    assert_eq!(user.email.as_str(), TEST_USER_EMAIL);
    assert_eq!(user.roles, vec![]);

    Ok(())
}

#[sqlx::test]
pub async fn test_delete_user_by_email(pool: PgPool) -> Result<(), String> {
    create_user(TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_EMPLOYEE_ID, &pool).await?;

    delete_user_by_email(TEST_USER_EMAIL, &pool).await?;

    let user = load_user(TEST_USER_EMAIL, &pool).await;

    if let Ok(user) = user {
        return Err(format!("User {} was not deleted", user.email));
    }

    Ok(())
}

#[sqlx::test]
pub async fn test_delete_user_by_employee_id(pool: PgPool) -> Result<(), String> {
    create_user(TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_EMPLOYEE_ID, &pool).await?;

    delete_user_by_employee_id(TEST_USER_EMPLOYEE_ID, &pool).await?;

    let user = load_user(TEST_USER_EMAIL, &pool).await;

    // User should not be found
    if let Ok(user) = user {
        return Err(format!("User {} was not deleted", user.email));
    }

    Ok(())
}
