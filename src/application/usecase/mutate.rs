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

    pub async fn mutate_text(
        &self,
        user_id: &UserId,
        new_diary: &Diary,
    ) -> Result<MutateResult, ApplicationError> {
        let target_id = new_diary.id();
        let prompt = get_prompt_by_id(target_id.to_id()).unwrap();
        let new_content = new_diary.content().to_value();
        let mut mutated_content = Vec::new();

        let user_data = match self.user_repository.find_by_id(user_id).await.unwrap() {
            Some(data) => data,
            None => {
                return Err(ApplicationError::NotFound {
                    entity_type: "User",
                    user_id: (*user_id.as_str()).to_string(),
                });
            },
        };

        let target_index = match &user_data.human_diary {
            Some(old_diary) => {
                let old_content = old_diary.content().to_value();
                find_first_different_index(new_content, old_content)
            },
            None => 0,
        };

        let api_url = "https://api.openai.com/v1/chat/completions";

        // target_indexがnew_contentの長さ以上の場合は、new_contentをそのまま返す
        if target_index >= new_content.len() {
            let mutated_diary = user_data.get_diary_by_id(target_id).unwrap();
            mutated_content = mutated_diary.content().to_value()[..new_content.len()].to_vec();
            return Ok(MutateResult {
                raw_contents: new_content.clone(),
                mutated_text: mutated_content.clone(),
                mutated_length: mutated_content.len(),
            });
        }

        for (i, raw_content) in new_content.clone().into_iter().enumerate() {
            if i < target_index {
                mutated_content.push(raw_content.clone());
                continue;
            }

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
                            mutated_content.push(process_output(processed_text));
                        } else {
                            mutated_content.push("Failed to mutate text.".to_string());
                        }
                    },
                    Err(_) => {
                        mutated_content.push("Error communicating with API.".to_string());
                    },
                }
            } else {
                mutated_content.push(raw_content.clone());
            }
        }

        Ok(MutateResult {
            raw_contents: new_content.clone(),
            mutated_text: mutated_content.clone(),
            mutated_length: mutated_content.len(),
        })
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

fn find_first_different_index(new_diary: &[String], old_diary: &[String]) -> usize {
    let min_len = std::cmp::min(new_diary.len(), old_diary.len());

    for i in 0..min_len {
        if new_diary[i] != old_diary[i] {
            return i;
        }
    }

    if new_diary.len() != old_diary.len() {
        return min_len;
    }

    new_diary.len()
}

fn process_output(input: String) -> String {
    if let Some(pos) = input.rfind("===") {
        input[(pos + 3)..].trim().to_string()
    } else {
        input.trim().to_string()
    }
}
