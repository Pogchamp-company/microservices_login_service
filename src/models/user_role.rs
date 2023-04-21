use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use sqlx::postgres::{PgHasArrayType, PgTypeInfo};

use crate::guards::user_token::UserTokenInfo;

#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRole {
    TaskManager,
    HumanResources,
    Director
}

impl PgHasArrayType for UserRole {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_user_role")
    }
}

pub fn has_permission_to_add_roles(user: &UserTokenInfo, roles: &Vec<UserRole>) -> bool {
    for role in roles {
        let can_add = match role {
            UserRole::TaskManager => {
                user.roles.contains(&UserRole::Director) || user.roles.contains(&UserRole::HumanResources)
            }
            UserRole::HumanResources => {
                user.roles.contains(&UserRole::Director)
            }
            UserRole::Director => {
                false
            }
        };
        if !can_add {
            return false;
        }
    }
    return true;
}

pub async fn add_roles(user_email: &str, roles: &Vec<UserRole>, pool: &PgPool) {
    let roles = roles.to_vec();
    sqlx::query!(r#"
        INSERT INTO user_to_role(user_email, role) SELECT $1, unnest($2::user_role[]) ON CONFLICT DO NOTHING;
    "#, user_email, roles as Vec<UserRole>).execute(pool).await.expect(&*format!("Can not add roles to user {}", user_email));
}