use rocket::response::status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::Serialize;

use crate::guards::user_token::{UserTokenError, UserTokenInfo};
use crate::models::user_role::UserRole;
use crate::views::base::ErrorJson;

#[derive(Debug, Serialize, JsonSchema)]
pub struct AccessTokenResponse {
    email: String,
    roles: Vec<UserRole>
}

/// # Check that provided token is valid and return all user roles
#[openapi]
#[get("/check")]
pub async fn check_access_token(current_user: Result<UserTokenInfo, UserTokenError>)
                            -> Result<Json<AccessTokenResponse>, status::Unauthorized<ErrorJson>> {
    let current_user = current_user?;
    return Ok(Json(AccessTokenResponse {
        email: current_user.email,
        roles: current_user.roles
    }));
}
