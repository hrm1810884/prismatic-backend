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
    pub pool: DbPool,
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
    async fn create(&self, user_id: &UserId) -> Result<(), DomainError> {
        let mut connection = self.get_connection()?;
        InternalUserRepository::create(user_id, &mut connection).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, DomainError> {
        let mut connection = self.get_connection()?;
        let user = InternalUserRepository::find_by_id(id, &mut connection).await?;
        Ok(user)
    }

    async fn update_diary(&self, user_id: &UserId, diary: &Diary) -> Result<(), DomainError> {
        let mut connection = self.get_connection()?;
        InternalUserRepository::update_diary(user_id, diary, &mut connection).await?;
        Ok(())
    }

    async fn update_result(
        &self,
        user_id: &UserId,
        is_public: bool,
        favorite_id: &DiaryId,
    ) -> Result<(), DomainError> {
        let mut connection = self.get_connection()?;
        InternalUserRepository::update_result(user_id, is_public, favorite_id, &mut connection)
            .await?;
        Ok(())
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
    pub async fn create(user_id: &UserId, conn: &mut MysqlConnection) -> Result<(), DomainError> {
        let current_time = Utc::now().naive_utc();
        let new_user = NewUser::new(user_id.as_str(), current_time, current_time);
        diesel::insert_into(user_schema::dsl::user)
            .values(new_user)
            .execute(conn)
            .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))?;
        Ok(())
    }

    pub async fn find_by_id(
        user_id: &UserId,
        conn: &mut MysqlConnection,
    ) -> Result<Option<User>, DomainError> {
        let user_row: Option<UserRow> = user_schema::dsl::user
            .filter(user_schema::user_id.eq(user_id.as_str()))
            .first::<UserRow>(conn)
            .optional()
            .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))?;

        let user: Option<User> = user_row.map(|row: UserRow| {
            let user_id = UserId::new(row.user_id).unwrap();
            let human_id = DiaryId::new(0).unwrap();
            let human_diary: Option<Diary> = match row.human_diary {
                Some(json_str) => {
                    let array: Vec<String> = serde_json::from_str(&json_str)
                        .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))
                        .unwrap();
                    let content = DiaryContent::new(array).unwrap();
                    Some(Diary::new(human_id.clone(), content).unwrap())
                },
                None => None,
            };
            let ai_diary_1: Option<Diary> = match row.ai_diary_1 {
                Some(json_str) => {
                    let array: Vec<String> = serde_json::from_str(&json_str)
                        .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))
                        .unwrap();
                    let content = DiaryContent::new(array).unwrap();
                    Some(Diary::new(human_id.clone(), content).unwrap())
                },
                None => None,
            };
            let ai_diary_2: Option<Diary> = match row.ai_diary_2 {
                Some(json_str) => {
                    let array: Vec<String> = serde_json::from_str(&json_str)
                        .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))
                        .unwrap();
                    let content = DiaryContent::new(array).unwrap();
                    Some(Diary::new(human_id.clone(), content).unwrap())
                },
                None => None,
            };
            let ai_diary_3: Option<Diary> = match row.ai_diary_3 {
                Some(json_str) => {
                    let array: Vec<String> = serde_json::from_str(&json_str)
                        .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))
                        .unwrap();
                    let content = DiaryContent::new(array).unwrap();
                    Some(Diary::new(human_id.clone(), content).unwrap())
                },
                None => None,
            };
            let ai_diary_4: Option<Diary> = match row.ai_diary_4 {
                Some(json_str) => {
                    let array: Vec<String> = serde_json::from_str(&json_str)
                        .map_err(|err| DomainError::InfrastructureError(anyhow::anyhow!(err)))
                        .unwrap();
                    let content = DiaryContent::new(array).unwrap();
                    Some(Diary::new(human_id.clone(), content).unwrap())
                },
                None => None,
            };
            let is_public = row.is_public;
            let favorite_id = row.favorite_id.map(|id| DiaryId::new(id).unwrap());
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

    pub async fn update_diary(
        user_id: &UserId,
        diary: &Diary,
        conn: &mut MysqlConnection,
    ) -> Result<(), DomainError> {
        let target_column = match diary.id().to_id() {
            0 => "human_diary",
            1 => "ai_diary_1",
            2 => "ai_diary_2",
            3 => "ai_diary_3",
            4 => "ai_diary_4",
            _ => {
                return Err(DomainError::InfrastructureError(anyhow::anyhow!(
                    "invalid target id"
                )))
            },
        };

        let query = format!("UPDATE user SET {} = ? WHERE user_id = ?", target_column);

        diesel::sql_query(query)
            .bind::<diesel::sql_types::Text, _>(&diary.content().to_json())
            .bind::<diesel::sql_types::Text, _>(user_id.as_str())
            .execute(conn)
            .map_err(|e| DomainError::InfrastructureError(anyhow::anyhow!(e)))?;

        Ok(())
    }

    pub async fn update_result(
        user_id: &UserId,
        is_public: bool,
        favorite_id: &DiaryId,
        conn: &mut MysqlConnection,
    ) -> Result<(), DomainError> {
        diesel::update(user_schema::table.filter(user_schema::user_id.eq(user_id.as_str())))
            .set((
                user_schema::is_public.eq(is_public),
                user_schema::favorite_id.eq(favorite_id.to_id()),
            ))
            .execute(conn)
            .map_err(|error| DomainError::InfrastructureError(anyhow::anyhow!(error)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use diesel::r2d2::ConnectionManager;
    use diesel::MysqlConnection;
    use tokio;

    use super::*;

    // テスト用のデータベース接続プールを作成
    fn create_test_db_pool() -> DbPool {
        dotenv::dotenv().ok();
        let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create test pool.")
    }

    #[tokio::test]
    async fn test_create_user() {
        let pool = create_test_db_pool();
        let repo = UserRepositoryImpl::new(pool);

        let user_id = UserId::new("test_user_id".to_string()).unwrap();

        let result = repo.create(&user_id).await;

        assert!(result.is_ok(), "Failed to create user: {:?}", result);
    }

    #[tokio::test]
    async fn test_find_user_by_id() {
        let pool = create_test_db_pool();
        let repo = UserRepositoryImpl::new(pool);

        let user_id = UserId::new("test_user_id".to_string()).unwrap();
        // repo.create(&user_id).await.unwrap();

        let found_user = repo.find_by_id(&user_id).await;

        assert!(found_user.is_ok(), "Failed to find user: {:?}", found_user);
        assert_eq!(found_user.unwrap().unwrap().id, user_id);
    }

    #[tokio::test]
    async fn test_update_diary() {
        let pool = create_test_db_pool();
        let repo = UserRepositoryImpl::new(pool);

        let user_id = UserId::new("test_user_id".to_string()).unwrap();
        // repo.create(&user_id).await.unwrap();

        let diary_id = DiaryId::new(1).unwrap();
        let diary_content = DiaryContent::new(vec!["Test diary entry".to_string()]).unwrap();
        let diary = Diary::new(diary_id, diary_content).unwrap();

        let result = repo.update_diary(&user_id, &diary).await;

        assert!(result.is_ok(), "Failed to update diary: {:?}", result);
    }

    #[tokio::test]
    async fn test_update_result() {
        let pool = create_test_db_pool();
        let repo = UserRepositoryImpl::new(pool);

        let user_id = UserId::new("test_user_id".to_string()).unwrap();
        // repo.create(&user_id).await.unwrap();

        let favorite_id = DiaryId::new(1).unwrap();
        let result = repo.update_result(&user_id, true, &favorite_id).await;

        assert!(result.is_ok(), "Failed to update result: {:?}", result);
    }
}
