use async_trait::async_trait;

use crate::domain::entity::diary::{Diary, DiaryId};
use crate::domain::entity::user::UserId;
use crate::domain::error::DomainError;

#[async_trait]
pub trait DiaryRepository: Send + Sync + 'static {
    async fn find_by_id(
        &self,
        user_id: &UserId,
        diary_id: &DiaryId,
    ) -> Result<Option<Diary>, DomainError>;
    async fn find_all(&self, user_id: &UserId) -> Result<Vec<Diary>, DomainError>;
    async fn update(&self, user_id: &UserId, diary: &Diary) -> Result<(), DomainError>;
}
