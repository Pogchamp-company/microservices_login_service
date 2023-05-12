use sqlx::PgPool;
use login_service::models::user::{create_user, load_user};
use login_service::models::user_role::{add_roles_by_email, delete_roles_by_employee_id, UserRole};

static TEST_USER_EMAIL: &str = "test@test.com";
static TEST_USER_PASSWORD: &str = "qwerty";
static TEST_USER_EMPLOYEE_ID: i32 = 1;

#[sqlx::test]
pub async fn test_delete_roles_by_employee_id(pool: PgPool) -> Result<(), String> {
    create_user(TEST_USER_EMAIL, TEST_USER_PASSWORD, TEST_USER_EMPLOYEE_ID, &pool).await?;

    let roles_to_add = [UserRole::HumanResources, UserRole::Director];

    add_roles_by_email(TEST_USER_EMAIL, &roles_to_add, &pool).await?;

    delete_roles_by_employee_id(TEST_USER_EMPLOYEE_ID, &pool).await?;

    let user = load_user(TEST_USER_EMAIL, &pool).await?;

    assert_eq!(user.roles, vec!());

    Ok(())
}
