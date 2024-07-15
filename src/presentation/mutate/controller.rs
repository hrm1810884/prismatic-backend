use actix_web::{web, HttpRequest, HttpResponse, Responder};
use log::error;

use super::response::MutateResult;
use crate::application::usecase::mutate::MutateUsecase;
use crate::auth::jwt::get_user_id_from_req;
use crate::domain::entity::diary::{Diary, DiaryContent, DiaryId};
use crate::infrastructure::database::user::UserRepositoryImpl;
use crate::presentation::mutate::request::MutateRequest;
use crate::presentation::mutate::response::MutateResponse;

pub async fn mutate_handler(
    req: HttpRequest,
    mutate_usecase: web::Data<MutateUsecase<UserRepositoryImpl>>,
    body: web::Json<MutateRequest>,
) -> impl Responder {
    let user_id = match get_user_id_from_req(req) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    let target_id = DiaryId::new(body.client_id).unwrap();
    let target_content = DiaryContent::new(body.target_text.clone()).unwrap();
    let target_diary = &Diary::new(target_id.clone(), target_content).unwrap();

    let result: MutateResult = mutate_usecase.mutate_text(target_diary).await;
    let mutated_content = DiaryContent::new(result.mutated_text.clone()).unwrap();

    if let Err(e) = mutate_usecase
        .save_diary(&user_id, &target_id, &mutated_content)
        .await
    {
        error!("Failed to save diary for user {}: {}", user_id.as_str(), e);
        return HttpResponse::InternalServerError().json("Error saving diary");
    }

    let response = MutateResponse { result };

    HttpResponse::Ok().json(response)
}

#[cfg(test)]
mod tests {
    use std::env;

    use actix_web::body::MessageBody;
    use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
    use actix_web::{test, web, App};
    use chrono::{Duration, Utc};
    use diesel::r2d2::ConnectionManager;
    use diesel::MysqlConnection;
    use serde_json::json;

    use super::mutate_handler;
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
        let openai_client = infrastructure::api::openai::OpenAiClient::new();
        let mutate_use_case = application::usecase::mutate::MutateUsecase::new(
            openai_client,
            user_repository.clone(),
        );

        App::new()
            .app_data(web::Data::new(mutate_use_case))
            .service(web::resource("/mutate").route(web::post().to(mutate_handler)))
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
    async fn test_mutate_handler() {
        println!("hogeeee");
        let app = test::init_service(setup_test_app()).await;

        let token = generate_test_jwt("test_user_id", b"your_secret_key");

        let request = test::TestRequest::post()
            .uri("/mutate")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(json!({
                "clientId": 3,
                "targetText": ["ここに書いていく．ここにも書いてく．"],
                "mutatedLength": 0
            }))
            .to_request();

        let response = test::call_service(&app, request).await;
        println!("result:{}", response.status());
        assert!(response.status().is_success());

        let response_body: serde_json::Value = test::read_body_json(response).await;
        print!("res: {}", response_body)
    }

    // 他のテストケースも同様に追加
}
