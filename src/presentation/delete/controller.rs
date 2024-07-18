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
