use actix_web::{web, HttpResponse, Responder};

use crate::application::services::mutate::MutateService;
use crate::presentation::mutate::request::MutateRequest;
use crate::presentation::mutate::response::MutateResponse;

pub async fn mutate_handler(
    mutate_service: web::Data<MutateService>,
    req: web::Json<MutateRequest>,
) -> impl Responder {
    let response: MutateResponse = mutate_service.mutate_text(&req).await;
    HttpResponse::Ok().json(response)
}
