use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateResultRequest {
    #[serde(rename = "favoriteId")]
    pub favorite_id: i32,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
}
