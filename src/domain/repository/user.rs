use async_trait::async_trait;

use crate::domain::entity::diary::{Diary, DiaryId};
use crate::domain::entity::user::{User, UserId};
use crate::domain::error::DomainError;

#[async_trait]
#[allow(dead_code)]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user_id: &UserId) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError>;
    async fn find_current_user(&self) -> Result<Option<User>, DomainError>;
    async fn update_diary(&self, user_id: &UserId, diary: &Diary) -> Result<(), DomainError>;
    async fn update_result(
        &self,
        user_id: &UserId,
        is_public: bool,
        favorite_id: &DiaryId,
    ) -> Result<(), DomainError>;
    async fn delete_user(&self, id: &UserId) -> Result<(), DomainError>;
}
