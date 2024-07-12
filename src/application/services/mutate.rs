use crate::infrastructure::api::openai::OpenAiClient;
use crate::presentation::mutate::request::MutateRequest;
use crate::presentation::mutate::response::{MutateResponse, MutateResult};

#[derive(Clone)]
pub struct MutateService {
    client: OpenAiClient,
}

impl MutateService {
    pub fn new(client: OpenAiClient) -> Self { Self { client } }

    pub async fn mutate_text(&self, req: &MutateRequest) -> MutateResponse {
        let prompts = [
            "入力テキストの感想・感情・意見を真逆の意味合いに書き換えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
            "入力テキストの感想・感情・意見など主観的な部分を楽観的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
            "入力テキストの感想・感情・意見など主観的な部分を悲観的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
            "入力テキストの感想・感情・意見など主観的な部分を自己拡張的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
        ];

        let window_id = req.client_id;
        let prompt = &prompts[window_id];
        let raw_contents = req.target_text.clone();
        let mut mutated_texts = Vec::new();

        let api_url = "https://api.openai.com/v1/chat/completions";

        for raw_content in &req.target_text {
            if !raw_content.trim().is_empty() {
                let content = format!(
                    "{} ただし、改行は入力文そのままにすること。\n ================ \n{}",
                    prompt, raw_content
                );

                let response = self
                    .client
                    .post(
                        api_url,
                        &serde_json::json!({
                            "model": "gpt-4-turbo",
                            "messages": [{"role": "user", "content": content}]
                        }),
                    )
                    .await;

                match response {
                    Ok(res) => {
                        let res_json = res.json::<serde_json::Value>().await.unwrap();
                        if let Some(mutated_text) =
                            res_json["choices"][0]["message"]["content"].as_str()
                        {
                            let processed_text = process_string(mutated_text.to_string());
                            mutated_texts.push(processed_text);
                        } else {
                            mutated_texts.push("Failed to mutate text.".to_string());
                        }
                    },
                    Err(_) => {
                        mutated_texts.push("Error communicating with API.".to_string());
                    },
                }
            }
        }

        MutateResponse {
            result: MutateResult {
                raw_contents: raw_contents.clone(),
                mutated_text: mutated_texts,
                mutated_length: req.mutated_length,
            },
        }
    }
}

fn process_string(input: String) -> String {
    if let Some(pos) = input.rfind("===") {
        input[(pos + 3)..].trim().to_string()
    } else {
        input.trim().to_string()
    }
}
