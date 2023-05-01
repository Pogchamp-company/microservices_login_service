use rocket::http::Status;
use rocket::response::status::{Custom, Unauthorized};
use rocket::serde::json::Json;
use rocket::serde::json::serde_json;
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
        return Unauthorized(Some(Json(
            ErrorJsonFormat {
                detail: serde_json::to_string(&value).unwrap()
            }
        )))
    }
}

impl From<UserTokenError> for Custom<ErrorJson> {
    fn from(value: UserTokenError) -> Self {
        return Custom(Status::Unauthorized, Json(
            ErrorJsonFormat {
                detail: serde_json::to_string(&value).unwrap()
            }
        ))
    }
}
