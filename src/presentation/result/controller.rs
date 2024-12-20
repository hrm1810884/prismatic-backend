use actix_web::{web, HttpRequest, HttpResponse, Responder};

use super::request::UpdateResultRequest;
use crate::application::usecase::result::UpdateResultUseCase;
use crate::auth::jwt::get_user_id_from_req;
use crate::domain::entity::diary::DiaryId;
use crate::infrastructure::database::user::UserRepositoryImpl;

pub async fn result_handler(
    req: HttpRequest,
    data: web::Data<UpdateResultUseCase<UserRepositoryImpl>>,
    body: web::Json<UpdateResultRequest>,
) -> impl Responder {
    let user_id = match get_user_id_from_req(req) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    // リクエストボディからfavorite_idを取得
    let favorite_id = DiaryId::new(body.favorite_id).unwrap();

    // ユースケースを実行
    match data
        .update_result(&user_id, body.is_public, &favorite_id)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Success"), // 成功時のレスポンス
        Err(_) => HttpResponse::InternalServerError().json("Error updating result"), // エラー時のレスポンス
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use actix_web::body::MessageBody;
    use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
    use actix_web::{http, test, web, App};
    use chrono::{Duration, Utc};
    use diesel::r2d2::ConnectionManager;
    use diesel::MysqlConnection;
    use serde_json::json;

    use super::result_handler;
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
        let update_result_use_case =
            application::usecase::result::UpdateResultUseCase::new(user_repository.clone());

        App::new()
            .app_data(web::Data::new(update_result_use_case))
            .service(web::resource("/result").route(web::post().to(result_handler)))
    }

    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        exp: usize,
    }
    fn generate_test_jwt(user_id: &str, secret: &[u8]) -> String {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(1))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_owned(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret),
        )
        .expect("token creation failed")
    }

    #[actix_rt::test]
    async fn test_result_handler_success() {
        println!("hogeeee");
        let app = test::init_service(setup_test_app()).await;

        let token = generate_test_jwt("3558d1e0-7997-43e5-9b2f-0a46292942c9", b"your_secret_key");

        let request = test::TestRequest::post()
            .uri("/result")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(json!({
                "isPublic": false,
                "favoriteId": 1
            }))
            .to_request();

        let response = test::call_service(&app, request).await;
        println!("result:{}", response.status());
        assert!(response.status().is_success());

        let response_body: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(response_body, json!("Success"));
    }

    #[actix_rt::test]
    async fn test_result_handler_unauthorized() {
        let app = test::init_service(setup_test_app()).await;

        let request = test::TestRequest::post()
            .uri("/result")
            .set_json(json!({
                "isPublic": true,
                "favoriteId": 1
            }))
            .to_request();

        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
    }

    // 他のテストケースも同様に追加
}
