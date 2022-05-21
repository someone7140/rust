use crate::service::search_service::get_search_condition_response;
use actix_web::{error::ErrorInternalServerError, get, HttpResponse, Responder};

#[get("/get_search_condition")]
pub async fn get_search_condition() -> impl Responder {
    let response = get_search_condition_response();

    return match response {
        Ok(r) => HttpResponse::Ok().content_type("application/json").json(r),
        Err(e) => ErrorInternalServerError(e.to_string()).into(),
    };
}
