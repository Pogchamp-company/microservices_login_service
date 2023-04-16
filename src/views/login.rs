use rocket::response::status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::models::user::load_user;
use crate::password_utils::{create_jwt, hash_password};
use crate::views::base::ErrorJson;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    login: String,
    password: String
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
}


#[get("/login", format = "json", data = "<login_request>")]
pub async fn login(login_request: Json<LoginRequest>,
                   pool: &rocket::State<PgPool>) -> Result<Json<LoginResponse>, status::Forbidden<Json<ErrorJson>>> {
    let user = load_user(&login_request.login, pool).await;

    let user = match user {
        Ok(user) => user,
        Err(detail) => {
            return Err(status::Forbidden(Some(
                Json(ErrorJson {
                    detail,
                })
            )))
        }
    };

    let hashed_password = hash_password(&login_request.password);
    if hashed_password != user.password {
        return Err(status::Forbidden(Some(
            Json(ErrorJson {
                detail: "Неправильный пароль".to_string(),
            })
        )));
    }

    Ok(Json(LoginResponse{
        token: create_jwt(&user.email, &user.password),
    }))
}