use crate::service::update_event_service::update_event_execute;
use actix_web::{error::ErrorInternalServerError, post, HttpResponse, Responder};

#[post("/update_event_info")]
pub async fn update_event_info() -> impl Responder {
    let response = update_event_execute();

    return match response {
        Ok(_r) => HttpResponse::Ok().json(""),
        Err(e) => ErrorInternalServerError(e.to_string()).into(),
    };
}
