use rocket::response::status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::user::{load_user, UserRole};
use crate::password_utils::{create_jwt, hash_password};
use crate::views::base::{ErrorJson, format_to_error_json};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    login: String,
    password: String
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
    roles: Vec<UserRole>
}


#[get("/login", format = "json", data = "<login_request>")]
pub async fn login(login_request: Json<LoginRequest>,
                   pool: &rocket::State<PgPool>) -> Result<Json<LoginResponse>, status::Forbidden<ErrorJson>> {
    let user = load_user(&login_request.login, pool).await;

    let user = match user {
        Ok(user) => user,
        Err(detail) => {
            return Err(status::Forbidden(format_to_error_json(detail)))
        }
    };

    let hashed_password = hash_password(&login_request.password);
    if hashed_password != user.password {
        return Err(status::Forbidden(format_to_error_json("Неправильный пароль".to_string())));
    }

    Ok(Json(LoginResponse{
        token: create_jwt(&user.email),
        roles: user.roles
    }))
}