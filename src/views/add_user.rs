use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{models, password_utils};
use crate::guards::user_token::UserTokenInfo;
use crate::models::user_role::{add_roles, UserRole};
use crate::models::user_role::has_permission_to_add_roles;
use crate::views::base::{ErrorJson, format_to_error_json};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddUserRequest {
    email: String,
    password: String,
    roles: Vec<UserRole>
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct AddUserResponse {
    token: String,
}

#[openapi]
#[post("/add_user", format = "json", data = "<add_user_request>")]
pub async fn add_user(add_user_request: Json<AddUserRequest>,
                      pool: &rocket::State<PgPool>,
                      current_user: UserTokenInfo) -> Result<Json<AddUserResponse>, status::Custom<ErrorJson>> {
    if !has_permission_to_add_roles(&current_user, &add_user_request.roles) {
        return Err(status::Custom(Status::Forbidden, format_to_error_json(
            "Вам недостаточно прав на создание пользователя с такими ролями".to_string()
        ).unwrap()))
    }

    let query_result = models::user::create_user(&add_user_request.email.clone(), &add_user_request.password, pool).await;

    if let Err(error_message) = query_result {
        return Err(status::Custom(Status::Conflict, format_to_error_json(error_message).unwrap()));
    }

    add_roles(&add_user_request.email, &add_user_request.roles, pool).await;

    return Ok(Json(AddUserResponse {
        token: password_utils::create_jwt(&add_user_request.email)
    }));
}
