use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::guards::user_token::{UserTokenError, UserTokenInfo};
use crate::models::user::user_exists;
use crate::models::user_role::UserRole;
use crate::views::base::{ErrorJson, format_to_error_json};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct UserExistsResponse {
    pub exists: bool,
}

/// # Check that user with provided email exists
#[openapi]
#[get("/exists/<email>")]
pub async fn check_access_token(current_user: Result<UserTokenInfo, UserTokenError>,
                                email: String,
                                pool: &rocket::State<PgPool>)
                                -> Result<Json<UserExistsResponse>, status::Custom<ErrorJson>> {
    current_user?;

    let result = user_exists(&email, pool).await.map_err(|error_message| {
        status::Custom(Status::InternalServerError, format_to_error_json(error_message).unwrap())
    })?;

    return Ok(Json(UserExistsResponse {
        exists: result,
    }));
}
