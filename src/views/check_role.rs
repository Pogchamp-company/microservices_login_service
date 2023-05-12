use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::Serialize;
use sqlx::PgPool;

use crate::guards::user_token::{UserTokenError, UserTokenInfo};
use crate::models::user_role::check_user_role;
use crate::models::user_role::UserRole;
use crate::views::base::{ErrorJson, format_to_error_json};

#[derive(Debug, Serialize, JsonSchema)]
pub struct CheckRoleResponse {
    has_access: bool,
}

/// # Check that you have provided role
#[openapi]
#[get("/check_role/<role>")]
pub async fn check_role(role: UserRole,
                        current_user: Result<UserTokenInfo, UserTokenError>,
                        pool: &rocket::State<PgPool>) -> Result<Json<CheckRoleResponse>, Unauthorized<ErrorJson>> {
    let current_user = current_user?;
    let user_has_role = check_user_role(&current_user.email, &role, pool).await;

    return if user_has_role {
        Ok(Json(CheckRoleResponse {
            has_access: true
        }))
    } else {
        Err(Unauthorized(format_to_error_json("Вы не обладаете правами доступа к этому разделу".to_string())))
    }
}
