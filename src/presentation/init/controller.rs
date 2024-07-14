use actix_web::{web, HttpResponse, Responder};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::response::InitResponse;
use crate::application::usecase::init::CreateUserUseCase;
use crate::domain::entity::user::UserId;
use crate::infrastructure::database::user::UserRepositoryImpl;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn init_handler(
    data: web::Data<CreateUserUseCase<UserRepositoryImpl>>,
) -> impl Responder {
    // 新しいユーザーIDを生成
    let user_id = Uuid::new_v4().to_string();

    // JWTトークンを生成
    let claims = Claims {
        sub: user_id.clone(),
        exp: 9999999999, // 適当に大きな値を設定
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"your_secret_key"),
    )
    .unwrap();

    // ユースケースを実行
    match data.create_user(&UserId::new(user_id).unwrap()).await {
        Ok(_) => HttpResponse::Ok().json(InitResponse { token }), // 成功時のレスポンス
        Err(_) => HttpResponse::InternalServerError().json("Error creating user"), // エラー時のレスポンス
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use actix_web::body::MessageBody;
    use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
    use actix_web::{test, web, App};
    use diesel::r2d2::ConnectionManager;
    use diesel::MysqlConnection;

    use super::init_handler;
    use crate::infrastructure::database::init::DbPool;
    use crate::{application, infrastructure};

    fn create_test_db_pool() -> DbPool {
        dotenv::dotenv().ok();
        let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create test pool.")
    }

    fn setup_test_app() -> App<
        impl ServiceFactory<
            ServiceRequest,
            Response = ServiceResponse<impl MessageBody>,
            Config = (),
            Error = actix_web::Error,
            InitError = (),
        >,
    > {
        // テスト用のデータベース接続プールの作成
        let pool = create_test_db_pool();

        // リポジトリとユースケースの設定
        let user_repository = infrastructure::database::user::UserRepositoryImpl::new(pool.clone());
        let create_user_use_case =
            application::usecase::init::CreateUserUseCase::new(user_repository.clone());

        App::new()
            .app_data(web::Data::new(create_user_use_case))
            .service(web::resource("/init").route(web::get().to(init_handler)))
    }

    #[actix_rt::test]
    async fn test_init_handler_success() {
        let app = test::init_service(setup_test_app()).await;

        let request = test::TestRequest::get().uri("/init").to_request();

        let response = test::call_service(&app, request).await;
        println!("result:{}", response.status());
        assert!(response.status().is_success());

        let response_body: serde_json::Value = test::read_body_json(response).await;
        println!("response: {}", response_body)
    }
}
