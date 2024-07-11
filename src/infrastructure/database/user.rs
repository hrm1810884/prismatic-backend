use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;
use serde_json;

use crate::domain::entity::diary::{Diary, DiaryContent, DiaryId};
use crate::domain::entity::user::{User, UserId};
use crate::domain::error::DomainError;
use crate::domain::repository::user::UserRepository;
use crate::infrastructure::database::init::DbPool;
use crate::infrastructure::database::models::NewUser;
use crate::schema::user::{self as user_schema};

pub struct UserRepositoryImpl {
    pool: DbPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: DbPool) -> Self { Self { pool } }

    fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, DomainError> {
        self.pool
            .get()
            .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: &User) -> Result<(), DomainError> {
        let mut connection = self.get_connection()?;
        InternalUserRepository::create(user, &mut connection).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        let mut connection = self.get_connection()?;
        let user = InternalUserRepository::find_by_id(id, &mut connection).await?;
        Ok(user)
    }
}

#[derive(Debug, Queryable)]
struct UserRow {
    user_id: String,
    human_diary: Option<String>,
    ai_diary_1: Option<String>,
    ai_diary_2: Option<String>,
    ai_diary_3: Option<String>,
    ai_diary_4: Option<String>,
    is_public: Option<bool>,
    favorite_id: Option<i32>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

pub struct InternalUserRepository;

impl InternalUserRepository {
    pub async fn create(user: &User, conn: &mut MysqlConnection) -> Result<(), DomainError> {
        let current_time = Utc::now().naive_utc();
        let new_user = NewUser::new(user.id.as_str(), current_time, current_time);
        diesel::insert_into(user_schema::dsl::user)
            .values(new_user)
            .execute(conn)
            .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))?;
        Ok(())
    }

    pub async fn find_by_id(
        id: &UserId,
        conn: &mut MysqlConnection,
    ) -> Result<Option<User>, DomainError> {
        let user_row: Option<UserRow> = user_schema::dsl::user
            .filter(user_schema::user_id.eq(id.as_str()))
            .first::<UserRow>(conn)
            .optional()
            .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))?;

        let user: Option<User> = user_row.map(|row: UserRow| {
            let user_id = UserId::new(row.user_id).unwrap();
            let human_id = DiaryId::new(0).unwrap();
            let human_diary: Diary = match row.human_diary {
                Some(json_str) => {
                    let array: Vec<String> = serde_json::from_str(&json_str)
                        .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))
                        .unwrap();
                    let content = DiaryContent::new(array).unwrap();
                    Some(Diary::new(human_id.clone(), content).unwrap())
                },
                None => None,
            }
            .unwrap();
            let ai_diary_1: Diary = match row.ai_diary_1 {
                Some(json_str) => {
                    let array: Vec<String> = serde_json::from_str(&json_str)
                        .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))
                        .unwrap();
                    let content = DiaryContent::new(array).unwrap();
                    Some(Diary::new(human_id.clone(), content).unwrap())
                },
                None => None,
            }
            .unwrap();
            let ai_diary_2: Diary = match row.ai_diary_2 {
                Some(json_str) => {
                    let array: Vec<String> = serde_json::from_str(&json_str)
                        .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))
                        .unwrap();
                    let content = DiaryContent::new(array).unwrap();
                    Some(Diary::new(human_id.clone(), content).unwrap())
                },
                None => None,
            }
            .unwrap();
            let ai_diary_3: Diary = match row.ai_diary_3 {
                Some(json_str) => {
                    let array: Vec<String> = serde_json::from_str(&json_str)
                        .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))
                        .unwrap();
                    let content = DiaryContent::new(array).unwrap();
                    Some(Diary::new(human_id.clone(), content).unwrap())
                },
                None => None,
            }
            .unwrap();
            let ai_diary_4: Diary = match row.ai_diary_4 {
                Some(json_str) => {
                    let array: Vec<String> = serde_json::from_str(&json_str)
                        .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))
                        .unwrap();
                    let content = DiaryContent::new(array).unwrap();
                    Some(Diary::new(human_id.clone(), content).unwrap())
                },
                None => None,
            }
            .unwrap();
            let is_public = row.is_public.unwrap();
            let favorite_id = DiaryId::new(row.favorite_id.unwrap()).unwrap();
            let created_at = row.created_at;
            let updated_at = row.updated_at;

            User::new(
                user_id,
                human_diary,
                ai_diary_1,
                ai_diary_2,
                ai_diary_3,
                ai_diary_4,
                is_public,
                favorite_id,
                created_at,
                updated_at,
            )
        });
        Ok(user)
    }
}

// #[cfg(test)]
// mod tests {
//     use std::env;

//     use diesel::mysql::MysqlConnection;
//     use diesel::r2d2::{ConnectionManager, Pool};
//     use dotenv::dotenv;

//     use super::*;

//     fn create_pool() -> DbPool {
//         dotenv().ok();
//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         let manager = ConnectionManager::<MysqlConnection>::new(database_url);
//         Pool::builder()
//             .build(manager)
//             .expect("Failed to create pool.")
//     }

//     #[tokio::test]
//     async fn test_user_repository() -> anyhow::Result<()> {
//         let pool = create_pool();
//         let repo = UserRepositoryImpl::new(pool);

//         let id = UserId::new(Uuid::new_v4().to_string())?;
//         let user = User::new(id.clone());

//         repo.create(&user).await?;
//         let fetched_user = repo.find_by_id(&id).await?;
//         assert!(fetched_user.is_some());

//         Ok(())
//     }
// }
