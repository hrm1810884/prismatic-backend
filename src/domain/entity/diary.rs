use getset::{Getters, Setters};
use validator_derive::Validate;

use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidDiaryId(i32);

impl ValidDiaryId {
    pub fn new(id: i32) -> Result<Self, DomainError> {
        if (0..=4).contains(&id) {
            Ok(ValidDiaryId(id))
        } else {
            Err(DomainError::Validation("invalid diary id".to_string()))
        }
    }

    pub fn value(&self) -> i32 { self.0 }
}

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct DiaryId {
    id: ValidDiaryId,
}

impl DiaryId {
    pub fn new(id: i32) -> Result<DiaryId, DomainError> {
        let valid_id = ValidDiaryId::new(id)?;
        Ok(DiaryId { id: valid_id })
    }

    pub fn is_human(&self) -> bool { self.id.value() == 0 }

    pub fn to_id(&self) -> i32 { self.id.value() }
}

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct DiaryContent {
    #[validate(length(min = 1))]
    value: String,
}

impl DiaryContent {
    pub fn new(content: String) -> Result<DiaryContent, DomainError> {
        Ok(DiaryContent { value: { content } })
    }
    pub fn to_value(&self) -> &String { &self.value }
    pub fn to_str(&self) -> &str { self.value.as_str() }
    pub fn to_json(&self) -> String { serde_json::to_string(&self.value).unwrap() }
    pub fn to_length(&self) -> i32 { self.value.chars().count() as i32 }
    pub fn get_from(&self, nth: i32) -> String {
        self.value.chars().skip(nth as usize).collect() // n文字目より後の文字列を取得
    }

    pub fn get_to(&self, nth: i32) -> String {
        self.value.chars().take(nth as usize).collect() // n文字目より前の文字列を取得
    }
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
