use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct InitResponse {
    pub token: String,
}
