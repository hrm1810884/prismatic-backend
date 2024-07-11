use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MutateRequest {
    #[serde(rename = "clientId")]
    pub client_id: usize,
    #[serde(rename = "targetText")]
    pub target_text: Vec<String>,
    #[serde(rename = "mutatedLength")]
    pub mutated_length: Option<usize>,
}

#[derive(Serialize, Debug)]
pub struct MutateResponse {
    pub result: MutateResult,
}

#[derive(Serialize, Debug)]
pub struct MutateResult {
    #[serde(rename = "rawContents")]
    pub raw_contents: Vec<String>,
    #[serde(rename = "mutatedText")]
    pub mutated_text: Vec<String>,
    #[serde(rename = "mutatedLength")]
    pub mutated_length: Option<usize>,
}
