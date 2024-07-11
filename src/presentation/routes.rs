use actix_web::web;

use crate::presentation::mutate::controller::mutate_handler;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/mutate").route(web::post().to(mutate_handler)));
}