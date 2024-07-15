use std::env;

use reqwest::Client;

#[derive(Clone)]
pub struct OpenAiClient {
    client: Client,
    api_key: String,
}

impl OpenAiClient {
    pub fn new() -> Self {
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn post(&self, body: &serde_json::Value) -> reqwest::Result<reqwest::Response> {
        let api_url = "https://api.openai.com/v1/chat/completions";
        self.client
            .post(api_url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await
    }
}
