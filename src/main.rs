use actix_cors::Cors;
use actix_web::{middleware as actix_middleware, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use infrastructure::database::init::create_pool;

mod application;
mod auth;
mod domain;
mod infrastructure;
mod middleware;
mod presentation;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let pool = create_pool();

    let user_repository = infrastructure::database::user::UserRepositoryImpl::new(pool.clone());
    let openai_client = infrastructure::api::openai::OpenAiClient::new();
    let mutate_service =
        application::usecase::mutate::MutateUsecase::new(openai_client, user_repository.clone());
    let update_result_use_case =
        application::usecase::result::UpdateResultUseCase::new(user_repository.clone());
    let create_user_use_case =
        application::usecase::init::CreateUserUseCase::new(user_repository.clone());
    let get_diary_use_case =
        application::usecase::diary::GetDiaryUseCase::new(user_repository.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(mutate_service.clone()))
            .app_data(actix_web::web::Data::new(update_result_use_case.clone()))
            .app_data(actix_web::web::Data::new(create_user_use_case.clone()))
            .app_data(actix_web::web::Data::new(get_diary_use_case.clone()))
            .wrap(actix_middleware::Logger::default())
            .wrap(middleware::Logging)
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .configure(presentation::routes::configure)
    })
    .workers(4)
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
