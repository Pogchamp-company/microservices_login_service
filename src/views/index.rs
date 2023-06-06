use rocket::serde::json::Json;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize, JsonSchema)]
pub struct IndexResponse {
    ok: bool
}

#[get("/")]
pub fn index() -> Json<IndexResponse> {
    Json(IndexResponse {
        ok: true
    })
}
