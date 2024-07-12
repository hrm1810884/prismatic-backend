use getset::{Getters, Setters};
use validator_derive::Validate;

use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct DiaryId {
    #[validate(range(min = 0, max = 4))]
    id: i32,
}

impl DiaryId {
    pub fn new(id: i32) -> Result<DiaryId, DomainError> { Ok(DiaryId { id }) }
    pub fn is_human(&self) -> bool { self.id == 0 }
    pub fn to_id(&self) -> i32 { self.id }
}

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct DiaryContent {
    #[validate(length(min = 1))]
    value: Vec<String>,
}

impl DiaryContent {
    pub fn new(content: Vec<String>) -> Result<DiaryContent, DomainError> {
        Ok(DiaryContent { value: { content } })
    }
    pub fn to_value(&self) -> &Vec<String> { &self.value }
    pub fn to_json(&self) -> String { serde_json::to_string(&self.value).unwrap() }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct Diary {
    #[getset(get = "pub")]
    id: DiaryId,
    #[getset(get = "pub", set = "pub")]
    content: DiaryContent,
}

impl Diary {
    pub fn new(id: DiaryId, content: DiaryContent) -> Result<Diary, DomainError> {
        Ok(Diary { id, content })
    }
}
