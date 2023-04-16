use rocket::response::status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::password_utils::get_email_from_token;
use crate::views::base::{ErrorJson, format_to_error_json};

#[derive(Debug, Deserialize)]
pub struct AccessTokenRequest {
    token: String,
}

#[derive(Debug, Serialize)]
pub struct AccessTokenResponse {
    email: String,
}

#[get("/check", format = "json", data = "<access_token_request>")]
pub async fn check_access_token(access_token_request: Json<AccessTokenRequest>,
                            pool: &rocket::State<PgPool>)
                            -> Result<Json<AccessTokenResponse>, status::Unauthorized<ErrorJson>> {

    return match get_email_from_token(&access_token_request.token) {
        Ok(email) => {
            Ok(Json(AccessTokenResponse {
                email
                // todo Return user roles
            }))
        }
        Err(error_message) => {
            Err(status::Unauthorized(format_to_error_json(error_message)))
        }
    };
}
