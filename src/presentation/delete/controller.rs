use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::usecase::delete::DeleteUsecase;
use crate::auth::jwt::get_user_id_from_req;
use crate::infrastructure::database::user::UserRepositoryImpl;

pub async fn delete_handler(
    req: HttpRequest,
    delete_usecase: web::Data<DeleteUsecase<UserRepositoryImpl>>,
) -> impl Responder {
    let user_id = match get_user_id_from_req(req) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    match delete_usecase.delete_user(&user_id).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::InternalServerError().json("Error Delete User"),
    }
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

    use super::delete_handler;
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
        let delete_user_use_case =
            application::usecase::delete::DeleteUsecase::new(user_repository.clone());

        App::new()
            .app_data(web::Data::new(delete_user_use_case))
            .service(web::resource("/delete").route(web::post().to(delete_handler)))
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
    async fn test_delete_handler() {
        let app = test::init_service(setup_test_app()).await;

        let token = generate_test_jwt("test_id", b"your_secret_key");

        let request = test::TestRequest::post()
            .uri("/delete")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request();

        let response = test::call_service(&app, request).await;
        println!("result:{}", response.status());
        assert!(response.status().is_success());
    }
}
