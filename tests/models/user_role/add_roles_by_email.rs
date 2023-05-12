use sqlx::PgPool;
use login_service::models::user::{create_user, load_user};
use login_service::models::user_role::{add_roles_by_email, UserRole};
use std::collections::HashSet;
use crate::models::user_role::{TEST_USER_EMAIL, TEST_USER_EMPLOYEE_ID, TEST_USER_PASSWORD};

#[sqlx::test]
pub async fn test_add_roles_by_email(pool: PgPool) -> Result<(), String> {
    create_user(TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_EMPLOYEE_ID, &pool).await?;

    let roles_to_add = [UserRole::HumanResources, UserRole::Director];

    add_roles_by_email(TEST_USER_EMAIL, &roles_to_add, &pool).await?;

    let user = load_user(TEST_USER_EMAIL, &pool).await?;

    assert_eq!(user.roles, roles_to_add);

    Ok(())
}

/// Check that addition of existing roles does not create duplicates of roles
#[sqlx::test]
pub async fn test_add_roles_by_email_with_overlap(pool: PgPool) -> Result<(), String> {
    create_user(TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_EMPLOYEE_ID, &pool).await?;

    let roles_to_add_first = [UserRole::HumanResources, UserRole::Director];
    let roles_to_add_second = [UserRole::Director, UserRole::TaskManager];

    add_roles_by_email(TEST_USER_EMAIL, &roles_to_add_first, &pool).await?;
    // UserRole::Director should not be duplicated
    add_roles_by_email(TEST_USER_EMAIL, &roles_to_add_second, &pool).await?;

    let user = load_user(TEST_USER_EMAIL, &pool).await?;

    // Union set of roles
    let roles_union: HashSet<UserRole> = HashSet::from(roles_to_add_first)
        .union(&HashSet::from(roles_to_add_second)).copied().collect();

    assert_eq!(HashSet::from_iter(user.roles), roles_union);

    Ok(())
}
