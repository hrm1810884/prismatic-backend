use uuid::Uuid;

use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId {
    id: Uuid,
}

impl UserId {
    pub fn new(id: Uuid) -> Result<UserId, DomainError> { Ok(UserId { id }) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: UserId,
}

impl User {
    pub fn new(id: UserId) -> Result<User, DomainError> { Ok(User { id }) }
}
