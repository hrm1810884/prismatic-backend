use async_trait::async_trait;

use crate::domain::entity::diary::{Diary, DiaryId};
use crate::domain::entity::user::{User, UserId};
use crate::domain::error::DomainError;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user_id: &UserId) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    async fn update_diary(&self, user_id: &UserId, diary: &Diary) -> Result<(), DomainError>;
    async fn update_result(
        &self,
        user_id: &UserId,
        is_public: bool,
        favorite_id: &DiaryId,
    ) -> Result<(), DomainError>;
}
