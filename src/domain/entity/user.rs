use chrono::NaiveDateTime;
use getset::{Getters, Setters};

use crate::domain::entity::diary::{Diary, DiaryId};
use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId {
    id: String,
}

impl UserId {
    pub fn new(id: String) -> Result<UserId, DomainError> { Ok(UserId { id }) }
    pub fn as_str(&self) -> &str { &self.id }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct User {
    #[getset(get = "pub")]
    pub id: UserId,
    #[getset(get = "pub", set = "pub")]
    pub human_diary: Option<Diary>,
    #[getset(get = "pub", set = "pub")]
    pub ai_diary_1: Option<Diary>,
    #[getset(get = "pub", set = "pub")]
    pub ai_diary_2: Option<Diary>,
    #[getset(get = "pub", set = "pub")]
    pub ai_diary_3: Option<Diary>,
    #[getset(get = "pub", set = "pub")]
    pub ai_diary_4: Option<Diary>,
    #[getset(get = "pub", set = "pub")]
    pub is_public: Option<bool>,
    #[getset(get = "pub", set = "pub")]
    pub favorite_id: Option<DiaryId>,
    #[getset(get = "pub", set = "pub")]
    pub created_at: NaiveDateTime,
    #[getset(get = "pub", set = "pub")]
    pub updated_at: NaiveDateTime,
}

impl User {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: UserId,
        human_diary: Option<Diary>,
        ai_diary_1: Option<Diary>,
        ai_diary_2: Option<Diary>,
        ai_diary_3: Option<Diary>,
        ai_diary_4: Option<Diary>,
        is_public: Option<bool>,
        favorite_id: Option<DiaryId>,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            human_diary,
            ai_diary_1,
            ai_diary_2,
            ai_diary_3,
            ai_diary_4,
            is_public,
            favorite_id,
            created_at,
            updated_at,
        }
    }
}
