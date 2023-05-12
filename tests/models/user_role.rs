use std::collections::HashSet;
use sqlx::PgPool;
use login_service::models::user::{create_user, delete_user_by_employee_id, load_user};
use login_service::models::user_role::{add_roles, UserRole};

static TEST_USER_EMAIL: &str = "test@test.com";
static TEST_USER_PASSWORD: &str = "qwerty";
static TEST_USER_EMPLOYEE_ID: i32 = 1;

#[sqlx::test]
pub async fn test_add_roles(pool: PgPool) -> Result<(), String> {
    create_user(TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_EMPLOYEE_ID, &pool).await?;

    let roles_to_add = [UserRole::HumanResources, UserRole::Director];

    add_roles(TEST_USER_EMAIL, &roles_to_add, &pool).await?;

    let user = load_user(TEST_USER_EMAIL, &pool).await?;

    assert_eq!(user.roles, roles_to_add);

    Ok(())
}

/// Check that addition of existing roles does not create duplicates of roles
#[sqlx::test]
pub async fn test_add_roles_with_overlap(pool: PgPool) -> Result<(), String> {
    create_user(TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_EMPLOYEE_ID, &pool).await?;

    let roles_to_add_first = [UserRole::HumanResources, UserRole::Director];
    let roles_to_add_second = [UserRole::Director, UserRole::TaskManager];

    add_roles(TEST_USER_EMAIL, &roles_to_add_first, &pool).await?;
    // UserRole::Director should not be duplicated
    add_roles(TEST_USER_EMAIL, &roles_to_add_second, &pool).await?;

    let user = load_user(TEST_USER_EMAIL, &pool).await?;

    // Union set of roles
    let roles_union: HashSet<UserRole> = HashSet::from(roles_to_add_first)
        .union(&HashSet::from(roles_to_add_second)).copied().collect();

    assert_eq!(HashSet::from_iter(user.roles), roles_union);

    Ok(())
}

#[sqlx::test]
pub async fn test_delete_user_with_roles(pool: PgPool) -> Result<(), String> {
    create_user(TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_EMPLOYEE_ID, &pool).await?;

    let roles_to_add = [UserRole::HumanResources, UserRole::Director];

    add_roles(TEST_USER_EMAIL, &roles_to_add, &pool).await?;

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
