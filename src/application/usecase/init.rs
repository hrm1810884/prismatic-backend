use crate::application::error::ApplicationError;
use crate::domain::entity::user::UserId;
use crate::domain::repository::user::UserRepository;

#[derive(Clone)]
pub struct CreateUserUseCase<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> CreateUserUseCase<R> {
    pub fn new(user_repository: R) -> Self { Self { user_repository } }

    pub async fn create_user(&self, user_id: &UserId) -> Result<(), ApplicationError> {
        self.user_repository.create(user_id).await?;

        Ok(())
    }
}
