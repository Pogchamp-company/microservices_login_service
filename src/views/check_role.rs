use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::user::check_user_role;
use crate::models::user_role::UserRole;
use crate::password_utils::get_email_from_token;
use crate::views::base::{ErrorJson, format_to_error_json};

#[derive(Debug, Deserialize)]
pub struct CheckRoleRequest {
    token: String,
    role: UserRole,
}

#[derive(Debug, Serialize)]
pub struct CheckRoleResponse {
    has_access: bool,
}

#[get("/check_role", format = "json", data = "<check_role_request>")]
pub async fn check_role(check_role_request: Json<CheckRoleRequest>,
                        pool: &rocket::State<PgPool>) -> Result<Json<CheckRoleResponse>, Unauthorized<ErrorJson>> {
    let email = get_email_from_token(&check_role_request.token);

    let email = match email {
        Ok(email) => email,

        Err(error_message) => {
            return Err(Unauthorized(format_to_error_json(error_message)));
        }
    };

    let user_has_role = check_user_role(&email, &check_role_request.role, pool).await;

    return if user_has_role {
        Ok(Json(CheckRoleResponse {
            has_access: true
        }))
    } else {
        Err(Unauthorized(format_to_error_json("Вы не обладаете правами доступа к этому разделу".to_string())))
    }
}
