use sqlx::PgPool;
use login_service::models::user::{create_user, delete_user_by_employee_id, load_user};
use crate::models::user::{TEST_USER_EMAIL, TEST_USER_EMPLOYEE_ID, TEST_USER_PASSWORD};

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
