use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use infrastructure::database::init::create_pool;

mod application;
mod auth;
mod domain;
mod infrastructure;
mod presentation;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = create_pool();

    let mutate_service = application::usecase::mutate::MutateService::new(
        infrastructure::api::openai::OpenAiClient::new(),
    );

    let user_repository = infrastructure::database::user::UserRepositoryImpl::new(pool.clone());
    let update_result_use_case =
        application::usecase::result::UpdateResultUseCase::new(user_repository.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(mutate_service.clone()))
            .app_data(actix_web::web::Data::new(update_result_use_case.clone()))
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .configure(presentation::routes::configure)
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
