use crate::application::error::ApplicationError;
use crate::domain::entity::user::UserId;
use crate::domain::repository::user::UserRepository;

#[derive(Clone)]
pub struct DeleteUsecase<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> DeleteUsecase<R> {
    pub fn new(user_repository: R) -> Self { Self { user_repository } }

    pub async fn delete_user(&self, user_id: &UserId) -> Result<(), ApplicationError> {
        self.user_repository.delete_user(user_id).await?;
        Ok(())
    }
}
