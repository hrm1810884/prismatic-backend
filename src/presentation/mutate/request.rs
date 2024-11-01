use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MutateRequest {
    #[serde(rename = "targetText")]
    pub target_text: String,
}
