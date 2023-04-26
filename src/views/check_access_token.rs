use rocket::response::status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::user::load_user;
use crate::models::user_role::UserRole;
use crate::password_utils::get_email_from_token;
use crate::views::base::{ErrorJson, format_to_error_json};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AccessTokenRequest {
    token: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct AccessTokenResponse {
    email: String,
    roles: Vec<UserRole>
}

#[openapi]
#[get("/check", format = "json", data = "<access_token_request>")]
pub async fn check_access_token(access_token_request: Json<AccessTokenRequest>,
                            pool: &rocket::State<PgPool>)
                            -> Result<Json<AccessTokenResponse>, status::Unauthorized<ErrorJson>> {
    let email = get_email_from_token(&access_token_request.token);

    let email = match email {
        Ok(email) => email,

        Err(error_message) => {
            return Err(status::Unauthorized(format_to_error_json(error_message)));
        }
    };

    let user = load_user(&email, pool).await;

    let user = match user {
        Ok(user) => user,

        Err(error_message) => {
            return Err(status::Unauthorized(format_to_error_json(error_message)));
        }
    };

    return Ok(Json(AccessTokenResponse {
        email,
        roles: user.roles
    }));
}
