use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct MutateResponse {
    pub result: MutateResult,
}

#[derive(Serialize, Debug, Clone)]
pub struct MutateResult {
    #[serde(rename = "mutatedLength")]
    pub mutated_length: usize,
}
