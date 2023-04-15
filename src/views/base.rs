use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorJson {
    pub detail: String,
}
