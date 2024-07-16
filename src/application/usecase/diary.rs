use crate::application::error::ApplicationError;
use crate::domain::entity::diary::{DiaryContent, DiaryId};
use crate::domain::entity::user::UserId;
use crate::domain::repository::user::UserRepository;

#[derive(Clone)]
pub struct GetDiaryUseCase<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> GetDiaryUseCase<R> {
    pub fn new(user_repository: R) -> Self { Self { user_repository } }

    pub async fn get_diary_by_id(
        &self,
        user_id: &UserId,
        diary_id: &DiaryId,
    ) -> Result<DiaryContent, ApplicationError> {
        let diary = match self.user_repository.find_by_id(user_id).await.unwrap() {
            Some(user_data) => user_data.get_diary_by_id(diary_id),
            None => None,
        }
        .unwrap();
        let diary_content = diary.content().clone();

        Ok(diary_content)
    }
}