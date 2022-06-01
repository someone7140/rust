use crate::model::db::event_info_collection::EventSearchMasterCollection;
use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc};
use chrono_tz::{Asia::Tokyo, Tz};
use std::error::Error;

pub fn gather_event_data(
    event_search_master: EventSearchMasterCollection,
    event_date: String,
    update_time: i64,
) -> Result<(), Box<dyn Error>> {
    return Ok(());
}
