use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DiaryResponse {
    pub result: DiaryResult,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DiaryResult {
    #[serde(rename = "diary")]
    pub diary: Vec<String>,
    #[serde(rename = "mutatedLength")]
    pub mutated_length: usize,
}
