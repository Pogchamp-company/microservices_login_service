use rocket::serde::json::Json;
use schemars::JsonSchema;
use serde::Serialize;

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
