use actix_web::{web, HttpResponse, Responder};

use crate::application::services::mutate::MutateService;
use crate::domain::models::mutate::MutateRequest;

pub async fn mutate_handler(
    mutate_service: web::Data<MutateService>,
    req: web::Json<MutateRequest>,
) -> impl Responder {
    let response = mutate_service.mutate_text(&req).await;
    HttpResponse::Ok().json(response)
}
