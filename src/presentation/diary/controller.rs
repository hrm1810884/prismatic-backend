use actix_web::{web, HttpResponse, Responder};

use super::request::DiaryRequestPath;
use super::response::{DiaryResponse, DiaryResult};
use crate::application::usecase::diary::GetDiaryUseCase;
use crate::domain::entity::diary::DiaryId;
use crate::infrastructure::database::user::UserRepositoryImpl;

pub async fn diary_handler(
    request_path: web::Path<DiaryRequestPath>,
    diary_usecase: web::Data<GetDiaryUseCase<UserRepositoryImpl>>,
) -> impl Responder {
    let diary_id = DiaryId::new(request_path.into_inner().client_id).unwrap();

    match diary_usecase.get_current_user_id(&diary_id).await {
        Ok(content) => {
            let diary = content.to_value().clone();
            HttpResponse::Ok().json(DiaryResponse {
                result: DiaryResult {
                    diary: diary.clone(),
                    mutated_length: diary.len(),
                },
            })
        },
        Err(_) => HttpResponse::InternalServerError().json("Get Diary Error"),
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
    use serde_json::from_slice;

    use super::diary_handler;
    use crate::infrastructure::database::init::DbPool;
    use crate::presentation::diary::response::DiaryResponse;
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
        let get_diary_use_case =
            application::usecase::diary::GetDiaryUseCase::new(user_repository.clone());

        App::new()
            .app_data(web::Data::new(get_diary_use_case))
            .service(web::resource("/diary/{clientId}").route(web::get().to(diary_handler)))
    }

    #[actix_rt::test]
    async fn test_get_diary_handler() {
        let app = test::init_service(setup_test_app()).await;

        let request = test::TestRequest::get().uri("/diary/3").to_request();

        let response = test::call_service(&app, request).await;
        println!("status:{}", response.status());
        assert!(response.status().is_success());
        let body = test::read_body(response).await;
        if let Ok(diary_response) = from_slice::<DiaryResponse>(&body) {
            println!(
                "response (formatted): {}",
                serde_json::to_string_pretty(&diary_response).unwrap()
            );
        } else {
            println!("failed to parse response body as DiaryResponse");
        }
    }

    // 他のテストケースも同様に追加
}
