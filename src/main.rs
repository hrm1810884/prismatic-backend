use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use infrastructure::database::init::create_pool;

mod application;
mod domain;
mod infrastructure;
mod presentation;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = create_pool();

    let mutate_service = application::services::mutate::MutateService::new(
        infrastructure::api::openai::OpenAiClient::new(),
    );

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(mutate_service.clone()))
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
