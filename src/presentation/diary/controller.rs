use actix_web::{web, HttpRequest, HttpResponse, Responder};

use super::request::DiaryRequest;
use super::response::{DiaryResponse, DiaryResult};
use crate::application::usecase::diary::GetDiaryUseCase;
use crate::auth::jwt::get_user_id_from_req;
use crate::domain::entity::diary::DiaryId;
use crate::infrastructure::database::user::UserRepositoryImpl;

pub async fn diary_handler(
    req: HttpRequest,
    diary_usecase: web::Data<GetDiaryUseCase<UserRepositoryImpl>>,
    body: web::Json<DiaryRequest>,
) -> impl Responder {
    let user_id = match get_user_id_from_req(req) {
        Ok(user_id) => user_id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    let diary_id = DiaryId::new(body.client_id).unwrap();
    match diary_usecase.get_diary_by_id(&user_id, &diary_id).await {
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
