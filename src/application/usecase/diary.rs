use crate::application::error::ApplicationError;
use crate::domain::entity::diary::{DiaryContent, DiaryId};
use crate::domain::repository::user::UserRepository;

#[derive(Clone)]
pub struct GetDiaryUseCase<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> GetDiaryUseCase<R> {
    pub fn new(user_repository: R) -> Self { Self { user_repository } }

    pub async fn get_current_user_diary(
        &self,
        diary_id: &DiaryId,
    ) -> Result<(DiaryContent, DiaryContent), ApplicationError> {
        let user_id = match self.user_repository.find_current_user().await.unwrap() {
            Some(user) => user.id().clone(),
            None => return Err(ApplicationError::Validation("Not Found".to_string())),
        };

        let ai_diary = match self.user_repository.find_by_id(&user_id).await.unwrap() {
            Some(user_data) => user_data.get_diary_by_id(diary_id),
            None => None,
        }
        .unwrap();
        let ai_diary_content = ai_diary.content().clone();

        let user_diary = match self.user_repository.find_by_id(&user_id).await.unwrap() {
            Some(user_data) => user_data.human_diary().clone(),
            None => None,
        }
        .unwrap();
        let user_diary_content = user_diary.content().clone();

        Ok((ai_diary_content, user_diary_content))
    }
}
