use crate::service::search_service;
use actix_web::{get, HttpResponse, Responder};

#[get("/get_search_condition")]
pub async fn get_search_condition() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(search_service::get_search_condition_response())
}
