use serde::Serialize;

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
