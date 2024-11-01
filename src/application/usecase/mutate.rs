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
        target_index: i32,
        user_data: &User,
        new_content: &DiaryContent,
    ) -> Result<(), ApplicationError> {
        let new_text = new_content.to_value();
        let prompt = get_prompt_by_id(target_id.to_id()).unwrap();
        let mut mutated_text = String::new();

        if target_index >= new_content.to_length() {
            // NOTE: 仕様上発生しないが、念のため 新しい日記の長さ以上の部分がtarget_indexに指定された場合長さだけ合わせる
            let mutated_diary = user_data.clone().get_diary_by_id(target_id).unwrap();
            mutated_text = mutated_diary.content().get_to(new_content.to_length());
            println!("{:?}", mutated_text);
        } else {
            let mutated_diary = user_data
                .clone()
                .get_diary_by_id(target_id)
                .unwrap_or_else(|| {
                    Diary::new(
                        target_id.clone(),
                        DiaryContent::new("".to_string()).unwrap(),
                    )
                    .unwrap()
                });
            mutated_text.push_str(mutated_diary.content().to_str());

            println!("{:?}", new_content.get_from(target_index));
            if !new_text.trim().is_empty() {
                let content = format!(
                    "{} ただし、改行は入力文そのままにすること。\n ================ \n{}",
                    prompt,
                    &new_content.get_from(target_index)
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
                            print!("{:?}", processed_text);
                            mutated_text.push_str(&process_output(processed_text));
                        } else {
                            mutated_text.push_str("Failed to mutate text.");
                        }
                    },
                    Err(_) => {
                        mutated_text.push_str("Error communicating with API.");
                    },
                }
            } else {
                mutated_text.push_str(new_text);
            }
        }

        let mutated_content = &DiaryContent::new(mutated_text).unwrap();
        self.save_diary(user_data.id(), target_id, mutated_content)
            .await?;

        Ok(())
    }

    pub async fn mutate_text(
        self: Arc<Self>,
        user_id: &UserId,
        new_content: &DiaryContent,
    ) -> Result<i32, ApplicationError> {
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
            Some(old_diary) => find_target_index(new_content, old_diary.content()),
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

        Ok(new_content.to_length())
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

// new_contentがold_contentの部分書き換えである場合には0を返す
// new_contentがold_contentのさらに後ろに追加されたものである場合にはold_contentの長さを返す
fn find_target_index(new_content: &DiaryContent, old_content: &DiaryContent) -> i32 {
    let new_text = new_content.to_value();
    let old_text = old_content.to_value();

    if new_text.starts_with(old_text) {
        old_content.to_length()
    } else {
        0
    }
}

fn process_output(input: String) -> String {
    if let Some(pos) = input.rfind("===") {
        input[(pos + 3)..].trim().to_string()
    } else {
        input.trim().to_string()
    }
}
