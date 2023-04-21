use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::guards::user_token::UserTokenInfo;
use crate::models::user::load_user;
use crate::models::user_role::{add_roles, UserRole};
use crate::models::user_role::has_permission_to_add_roles;
use crate::views::base::{ErrorJson, format_to_error_json};

#[derive(Debug, Deserialize)]
pub struct AddRolesRequest {
    login: String,
    roles: Vec<UserRole>
}

#[derive(Debug, Serialize)]
pub struct AddRolesResponse {
    ok: bool,
}

#[post("/add_roles", format = "json", data = "<add_roles_request>")]
pub async fn add_roles_view(add_roles_request: Json<AddRolesRequest>,
                      pool: &rocket::State<PgPool>,
                      current_user: UserTokenInfo) -> Result<Json<AddRolesResponse>, status::Custom<ErrorJson>> {
    if !has_permission_to_add_roles(&current_user, &add_roles_request.roles) {
        return Err(status::Custom(Status::Forbidden, format_to_error_json(
            "Вам недостаточно прав на создание пользователя с такими ролями".to_string()
        ).unwrap()))
    }

    let user = load_user(&add_roles_request.login, pool).await;

    if let Err(error_message) = user {
        return Err(status::Custom(Status::BadRequest, format_to_error_json(error_message).unwrap()));
    }

    add_roles(&add_roles_request.login, &add_roles_request.roles, pool).await;

    return Ok(Json(AddRolesResponse {
        ok: true
    }));
}