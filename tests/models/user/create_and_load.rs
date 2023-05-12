use sqlx::PgPool;
use login_service::models::user::{create_user, load_user};
use crate::models::user::{TEST_USER_EMAIL, TEST_USER_EMPLOYEE_ID, TEST_USER_PASSWORD};

#[sqlx::test]
pub async fn test_create_and_load_user(pool: PgPool) -> Result<(), String> {
    create_user(TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_EMPLOYEE_ID, &pool).await?;

    let user = load_user(TEST_USER_EMAIL, &pool).await?;

    assert_eq!(user.email.as_str(), TEST_USER_EMAIL);
    assert_eq!(user.roles, vec![]);

    Ok(())
}
