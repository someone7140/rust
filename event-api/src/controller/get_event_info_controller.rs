use crate::service::get_event_service::get_event_info_list;
use crate::service::get_event_service::get_event_info_master;
use actix_web::web::Query;
use actix_web::{error::ErrorInternalServerError, get, HttpResponse, Responder};
use serde::Deserialize;

#[get("/get_event_info_master")]
pub async fn get_event_master() -> impl Responder {
    let response = get_event_info_master();
    return HttpResponse::Ok()
        .content_type("application/json")
        .json(response);
}

#[derive(Clone, Deserialize)]
pub struct GetEventListQuery {
    location_key: String,
    event_date: String,
}

#[get("/get_event_list")]
pub async fn get_event_list(query: Query<GetEventListQuery>) -> impl Responder {
    let location_key = &query.location_key;
    let event_date = &query.event_date;

    let response = get_event_info_list(
        (location_key.clone()).to_string(),
        (event_date.clone()).to_string(),
    )
    .await;
    return match response {
        Ok(_r) => HttpResponse::Ok().json(_r),
        Err(e) => ErrorInternalServerError(e.to_string()).into(),
    };
}
