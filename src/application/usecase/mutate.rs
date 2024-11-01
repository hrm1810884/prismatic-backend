use std::sync::Arc;

use tokio::task;

use crate::application::error::ApplicationError;
use crate::domain::entity::diary::{Diary, DiaryContent, DiaryId};
use crate::domain::entity::user::{User, UserId};
use crate::domain::repository::user::UserRepository;
use crate::infrastructure::api::openai::OpenAiClient;

static PROMPTS: [&str; 4] = [
    "入力テキストの感想・感情・意見を真逆の意味合いに書き換えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
    "入力テキストの感想・感情・意見など主観的な部分を楽観的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
    "入力テキストの感想・感情・意見など主観的な部分を悲観的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
    "入力テキストの感想・感情・意見など主観的な部分を自己拡張的に書き替えてください。但し、口調・固有名詞と客観的事実は変更しないでください。",
];

#[derive(Clone)]
pub struct MutateUsecase<R: UserRepository> {
    client: Arc<OpenAiClient>,
    user_repository: Arc<R>,
}

impl<R: UserRepository> MutateUsecase<R> {
    pub fn new(client: OpenAiClient, user_repository: R) -> Self {
        Self {
            client: Arc::new(client),
            user_repository: Arc::new(user_repository),
        }
    }

    pub async fn process_mutation_by_id(
        self: Arc<Self>,
        target_id: &DiaryId,
        target_index: usize,
        user_data: &User,
        new_content: &DiaryContent,
    ) -> Result<(), ApplicationError> {
        let new_text = new_content.to_value();
        let prompt = get_prompt_by_id(target_id.to_id()).unwrap();
        let mut mutated_text = Vec::new();

        if target_index >= new_text.len() {
            let mutated_diary = user_data.clone().get_diary_by_id(target_id).unwrap();
            mutated_text = mutated_diary.content().to_value()[..new_text.len()].to_vec();
        } else {
            for (i, raw_text) in new_text.clone().into_iter().enumerate() {
                if i < target_index {
                    let mutated_diary = user_data.clone().get_diary_by_id(target_id).unwrap();
                    mutated_text.push(mutated_diary.content().to_value()[i].clone());
                    continue;
                }

                if !raw_text.trim().is_empty() {
                    let content = format!(
                        "{} ただし、改行は入力文そのままにすること。\n ================ \n{}",
                        prompt, raw_text
                    );

                    let response = self
                        .client
                        .post(&serde_json::json!({
                            "model": "gpt-4-turbo",
                            "messages": [{"role": "user", "content": content}]
                        }))
                        .await;

                    match response {
                        Ok(res) => {
                            let res_json = res.json::<serde_json::Value>().await.unwrap();
                            if let Some(mutated_response) =
                                res_json["choices"][0]["message"]["content"].as_str()
                            {
                                let processed_text = process_output(mutated_response.to_string());
                                mutated_text.push(process_output(processed_text));
                            } else {
                                mutated_text.push("Failed to mutate text.".to_string());
                            }
                        },
                        Err(_) => {
                            mutated_text.push("Error communicating with API.".to_string());
                        },
                    }
                } else {
                    mutated_text.push(raw_text.clone());
                }
            }
        }

        println!("{:?}", mutated_text);
        let mutated_content = &DiaryContent::new(mutated_text).unwrap();
        self.save_diary(user_data.id(), target_id, mutated_content)
            .await?;

        Ok(())
    }

    pub async fn mutate_text(
        self: Arc<Self>,
        user_id: &UserId,
        new_content: &DiaryContent,
    ) -> Result<usize, ApplicationError> {
        let new_text = new_content.to_value();

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
                let old_text = old_diary.content().to_value();
                find_first_different_index(new_text, old_text)
            },
            None => 0,
        };

        let ids = vec![1, 2, 3, 4];
        let mut tasks = vec![];
        let shared_self = Arc::clone(&self);
        for id in ids {
            let shared_self = Arc::clone(&shared_self);
            let user_data = user_data.clone();
            let new_content = new_content.clone();
            tasks.push(task::spawn(async move {
                let target_id = &DiaryId::new(id).unwrap();
                shared_self
                    .process_mutation_by_id(target_id, target_index, &user_data, &new_content)
                    .await
            }));
        }

        for task in tasks {
            match task.await {
                Ok(result) => result?,
                Err(e) => return Err(ApplicationError::Unexpected(e.to_string())), // or another variant that suits the error
            }
        }

        let human_diary_id = &DiaryId::new(0).unwrap();
        self.save_diary(user_id, human_diary_id, new_content)
            .await?;

        Ok(new_text.len())
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
    if id >= 1 && (id as usize) <= PROMPTS.len() {
        Some(PROMPTS[(id - 1) as usize])
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
