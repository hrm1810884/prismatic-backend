use thiserror::Error;

use crate::domain::error::DomainError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("{0}")]
    Validation(String),
    #[error(r#"{entity_type} was not found for user_id "{user_id}"."#)]
    NotFound {
        entity_type: &'static str,
        user_id: String,
    },
    #[error(transparent)]
    InfrastructureError(anyhow::Error),
    #[error("{0}")]
    Unexpected(String),
}

impl From<DomainError> for ApplicationError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::Validation(message) => ApplicationError::Validation(message),
            DomainError::NotFound {
                entity_type,
                user_id,
            } => ApplicationError::NotFound {
                entity_type,
                user_id,
            },
            DomainError::InfrastructureError(_) => {
                ApplicationError::InfrastructureError(anyhow::Error::new(err))
            },
            DomainError::Unexpected(message) => ApplicationError::Unexpected(message),
        }
    }
}
