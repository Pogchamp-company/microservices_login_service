use sqlx::PgPool;
use login_service::models::user::{create_user, delete_user_by_email, delete_user_by_employee_id, load_user};
use login_service::models::user_role::{add_roles_by_email, UserRole};
use crate::models::user::{TEST_USER_EMAIL, TEST_USER_EMPLOYEE_ID, TEST_USER_PASSWORD};

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
pub async fn test_delete_user_with_roles(pool: PgPool) -> Result<(), String> {
    create_user(TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_EMPLOYEE_ID, &pool).await?;

    let roles_to_add = [UserRole::HumanResources, UserRole::Director];

    add_roles_by_email(TEST_USER_EMAIL, &roles_to_add, &pool).await?;

    delete_user_by_employee_id(TEST_USER_EMPLOYEE_ID, &pool).await?;

    let user = load_user(TEST_USER_EMAIL, &pool).await;

    // User should not be found
    if let Ok(user) = user {
        return Err(format!("User {} was not deleted", user.email));
    }

    let user_to_role_entries = sqlx::query!(r#"
            SELECT count(role) as "count!" FROM user_to_role WHERE user_email = $1
        "#, TEST_USER_EMAIL).fetch_one(&pool).await;

    let user_to_role_entries = user_to_role_entries.map_err(|err| -> String { err.to_string() })?;

    // Corresponding roles should be deleted
    assert_eq!(user_to_role_entries.count, 0);

    Ok(())
}
