use rocket::response::status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{models, password_utils};
use crate::views::base::{ErrorJson, format_to_error_json};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    login: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    token: String,
}

#[post("/register", format = "json", data = "<user_login>")]
pub async fn register(user_login: Json<RegisterRequest>,
                      pool: &rocket::State<PgPool>) -> Result<Json<RegisterResponse>, status::Conflict<ErrorJson>> {
    let hashed_password = password_utils::hash_password(&user_login.password);
    let query_result = models::user::create_user(&user_login.login.clone(), &hashed_password, pool).await;

    if let Err(..) = query_result {
        return Err(status::Conflict(format_to_error_json("Введённый email уже существует".to_string())));
    }

    return Ok(Json(RegisterResponse {
        token: password_utils::create_jwt(&user_login.login)
    }));
}
