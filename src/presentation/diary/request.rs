use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct DiaryRequestPath {
    #[serde(rename = "clientId")]
    pub client_id: i32,
}
