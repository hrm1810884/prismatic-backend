use crate::application::error::ApplicationError;
use crate::domain::entity::diary::{Diary, DiaryContent, DiaryId};
use crate::domain::entity::user::UserId;
use crate::domain::repository::user::UserRepository;
use crate::infrastructure::api::openai::OpenAiClient;
use crate::presentation::mutate::response::MutateResult;

static PROMPTS: [&str; 4] = [
    "入力テキストの感想・感情・意見を真逆の意味合いに書き換えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
    "入力テキストの感想・感情・意見など主観的な部分を楽観的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
    "入力テキストの感想・感情・意見など主観的な部分を悲観的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
    "入力テキストの感想・感情・意見など主観的な部分を自己拡張的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
];

#[derive(Clone)]
pub struct MutateUsecase<R: UserRepository> {
    client: OpenAiClient,
    user_repository: R,
}

impl<R: UserRepository> MutateUsecase<R> {
    pub fn new(client: OpenAiClient, user_repository: R) -> Self {
        Self {
            client,
            user_repository,
        }
    }

    pub async fn mutate_text(&self, target_diary: &Diary) -> MutateResult {
        let prompt = get_prompt_by_id(target_diary.id().to_id()).unwrap();
        let raw_contents = target_diary.content().to_value();
        let mut mutated_texts = Vec::new();

        let api_url = "https://api.openai.com/v1/chat/completions";

        for raw_content in raw_contents.clone() {
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
                            let processed_text = process_output(mutated_text.to_string());
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

        MutateResult {
            raw_contents: raw_contents.clone(),
            mutated_text: mutated_texts.clone(),
            mutated_length: mutated_texts.len(),
        }
    }

    pub async fn save_diary(
        &self,
        user_id: &UserId,
        diary_id: &DiaryId,
        diary: &DiaryContent,
    ) -> Result<(), ApplicationError> {
        self.user_repository
            .update_diary(
                user_id,
                &Diary::new(diary_id.clone(), diary.clone()).unwrap(),
            )
            .await?;

        Ok(())
    }
}

fn get_prompt_by_id(id: i32) -> Option<&'static str> {
    if id >= 0 && (id as usize) < PROMPTS.len() {
        Some(PROMPTS[id as usize])
    } else {
        None
    }
}

fn find_first_different_index(new_diary: &[String], old_diary: &[String]) -> Option<usize> {
    let min_len = std::cmp::min(new_diary.len(), old_diary.len());

    for i in 0..min_len {
        if new_diary[i] != old_diary[i] {
            return Some(i);
        }
    }

    if new_diary.len() != old_diary.len() {
        return Some(min_len);
    }

    None
}

fn process_output(input: String) -> String {
    if let Some(pos) = input.rfind("===") {
        input[(pos + 3)..].trim().to_string()
    } else {
        input.trim().to_string()
    }
}
