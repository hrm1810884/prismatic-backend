use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DiaryResponse {
    pub result: DiaryResult,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DiaryResult {
    #[serde(rename = "diary")]
    pub diary: String,
    #[serde(rename = "mutatedLength")]
    pub mutated_length: MutatedLength,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MutatedLength {
    pub human: i32,
    pub ai: i32,
}
