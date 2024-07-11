use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("{0}")]
    Validation(String),
    #[error(r#"{entity_type} was not found for user_id "{user_id}"."#)]
    NotFound {
        entity_type: &'static str,
        user_id: String,
    },
    #[error("{0}")]
    Unexpected(String),
}

impl From<ValidationErrors> for DomainError {
    fn from(err: ValidationErrors) -> Self { DomainError::Validation(err.to_string()) }
}
