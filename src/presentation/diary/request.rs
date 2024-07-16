use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct DiaryRequest {
    #[serde(rename = "clientId")]
    pub client_id: i32,
}
