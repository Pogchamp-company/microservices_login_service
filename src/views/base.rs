use rocket::http::Status;
use rocket::response::status::{Custom, Unauthorized};
use rocket::serde::json::Json;
use schemars::JsonSchema;
use serde::Serialize;

use crate::guards::user_token::UserTokenError;

#[derive(Debug, Serialize, JsonSchema)]
pub struct ErrorJsonFormat {
    pub detail: String,
}

pub type ErrorJson = Json<ErrorJsonFormat>;

pub fn format_to_error_json(detail: String) -> Option<ErrorJson> {
    Some(
        Json(ErrorJsonFormat {
            detail
        })
    )
}

impl From<UserTokenError> for Unauthorized<Json<ErrorJsonFormat>> {
    fn from(value: UserTokenError) -> Self {
        let error_message = match value {
            UserTokenError::Missing { message }
            | UserTokenError::BadCount { message }
            | UserTokenError::Parse { message }
            | UserTokenError::EmailNotFound { message } => {
                message
            }
        };

        return Unauthorized(Some(Json(
            ErrorJsonFormat {
                detail: error_message
            }
        )));
    }
}

impl From<UserTokenError> for Custom<ErrorJson> {
    fn from(value: UserTokenError) -> Self {
        let error_message = match value {
            UserTokenError::Missing { message }
            | UserTokenError::BadCount { message }
            | UserTokenError::Parse { message }
            | UserTokenError::EmailNotFound { message } => {
                message
            }
        };

        return Custom(Status::Unauthorized, Json(
            ErrorJsonFormat {
                detail: error_message
            }
        ));
    }
}
