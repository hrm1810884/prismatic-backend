use crate::application::error::ApplicationError;
use crate::domain::entity::diary::DiaryId;
use crate::domain::entity::user::UserId;
use crate::domain::repository::user::UserRepository;

#[derive(Clone)]
pub struct UpdateResultUseCase<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> UpdateResultUseCase<R> {
    pub fn new(user_repository: R) -> Self { Self { user_repository } }

    pub async fn update_result(
        &self,
        user_id: &UserId,
        is_public: bool,
        favorite_id: &DiaryId,
    ) -> Result<(), ApplicationError> {
        self.user_repository
            .update_result(user_id, is_public, favorite_id)
            .await?;

        Ok(())
    }
}
