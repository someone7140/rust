use actix_cors::Cors;
use async_graphql::*;
use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::{Asia::Tokyo, Tz, UTC};

use crate::graphql_object::horse_enum::ErrorType;

pub fn get_cors_setting(origin: &String) -> Cors {
    Cors::default()
        .allowed_origin(origin)
        .allowed_methods(vec!["GET", "POST", "PUT", "OPTIONS", "DELETE"])
        .allowed_header(actix_web::http::header::CONTENT_TYPE)
        .allowed_headers(vec![
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::ACCEPT,
        ])
        .supports_credentials()
}

// 日付の文字列からutc日付のDateTime
pub fn get_utc_date_from_date_str(date_str: &String) -> Result<DateTime<Tz>> {
    let native_date =
        match NaiveDateTime::parse_from_str(&(date_str.to_string() + " 00:00:00"), "%Y/%m/%d %T") {
            Ok(date) => date,
            Err(error) => {
                return Err(Error::new(error.to_string())
                    .extend_with(|_, e| e.set("type", ErrorType::BadRequest)))
            }
        };
    match Tokyo.from_local_datetime(&native_date) {
        chrono::LocalResult::Single(date) => Ok(date.with_timezone(&UTC)),
        _ => {
            return Err(Error::new("Parse jst time error")
                .extend_with(|_, e| e.set("type", ErrorType::BadRequest)))
        }
    }
}

// timestampからJSTのDateTimeを取得
pub fn get_jst_date_from_timestamp_millis(timestamp_millis: i64) -> Result<DateTime<Tz>> {
    let date_utc = match DateTime::from_timestamp_millis(timestamp_millis) {
        Some(date) => date.naive_utc(),
        None => {
            return Err(Error::new("Date parse error")
                .extend_with(|_, e| e.set("type", ErrorType::BadRequest)))
        }
    };
    Ok(Tokyo.from_utc_datetime(&date_utc))
}
