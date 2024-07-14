use actix_web::web;

use super::result::controller::result_handler;
use crate::presentation::mutate::controller::mutate_handler;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/mutate").route(web::post().to(mutate_handler)));
    cfg.service(web::resource("/result").route(web::post().to(result_handler)));
}
