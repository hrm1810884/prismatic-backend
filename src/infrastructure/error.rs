use diesel::result::Error as DieselError;

use crate::domain::error::DomainError;

impl From<DieselError> for DomainError {
    fn from(error: DieselError) -> Self {
        DomainError::InfrastructureError(anyhow::Error::new(error))
    }
}
