use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MutateRequest {
    #[serde(rename = "clientId")]
    pub client_id: i32,
    #[serde(rename = "targetText")]
    pub target_text: Vec<String>,
    #[serde(rename = "mutatedLength")]
    pub mutated_length: Option<i32>,
}
