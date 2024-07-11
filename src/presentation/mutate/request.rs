use serde::Deserialize;

#[derive(Deserialize)]
pub struct MutateRequest {
    #[serde(rename = "clientId")]
    pub client_id: usize,
    #[serde(rename = "targetText")]
    pub target_text: Vec<String>,
    #[serde(rename = "mutatedLength")]
    pub mutated_length: Option<usize>,
}