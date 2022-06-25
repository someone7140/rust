use crate::model::db::event_info_collection::{
    EventSearchMasterCollection, EventUpdateHistoryCollection,
};
use crate::repository::mongodb_client;
use crate::util::date_util;
use chrono::{DateTime, Duration};
use chrono_tz::Tz;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use std::error::Error;

pub fn get_event_search_master() -> Result<Vec<EventSearchMasterCollection>, Box<dyn Error>> {
    let col = mongodb_client::get_mongodb_db_connection()?
        .collection::<EventSearchMasterCollection>("event_search_master");
    let results = col.find(None, None)?.flatten().collect();
    return Ok(results);
}

pub fn get_event_update_history() -> Result<Vec<EventUpdateHistoryCollection>, Box<dyn Error>> {
    let col = mongodb_client::get_mongodb_db_connection()?
        .collection::<EventUpdateHistoryCollection>("event_update_history");
    let find_options = FindOptions::builder()
        .sort(doc! { "location_key": 1, "update_time": 1, "event_date": 1  })
        .build();
    let results = col.find(None, find_options)?.flatten().collect();
    return Ok(results);
}

pub fn set_init_event_update_history(
    location_key: String,
    target_date: DateTime<Tz>,
    register_date: i32,
) -> Result<Vec<EventUpdateHistoryCollection>, Box<dyn Error>> {
    // insertの元データ
    let register_date_vec: Vec<i32> = (1..=register_date).collect();
    let insert_docs: Vec<EventUpdateHistoryCollection> = register_date_vec
        .into_iter()
        .map(|d| {
            let insert_date = target_date + Duration::days(d.into());
            return EventUpdateHistoryCollection {
                location_key: location_key.clone(),
                event_date: date_util::format_jst_date(insert_date, "%Y-%m-%d"),
                update_time: 1.into(),
            };
        })
        .collect();
    let return_docs = insert_docs.to_vec();
    // DBの登録処理
    let col = mongodb_client::get_mongodb_db_connection()?
        .collection::<EventUpdateHistoryCollection>("event_update_history");
    col.insert_many(insert_docs, None)?;
    return Ok(return_docs);
}

pub fn update_time_event_update_history(
    location_key: String,
    event_date: String,
    update_time: i64,
) -> Result<(), Box<dyn Error>> {
    let col = mongodb_client::get_mongodb_db_connection()?
        .collection::<EventUpdateHistoryCollection>("event_update_history");
    col.update_one(
        doc! {
            "location_key": location_key, "event_date": event_date
        },
        doc! {
            "$set": { "update_time": update_time }
        },
        None,
    )?;
    return Ok(());
}

pub fn delete_event_update_history(
    location_key: String,
    event_dates: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let col = mongodb_client::get_mongodb_db_connection()?
        .collection::<EventUpdateHistoryCollection>("event_update_history");
    let delete_target_query = doc! {
        "$and": [
            { "location_key": location_key },
            { "event_date": { "$in": event_dates } }
        ]
    };
    col.delete_many(delete_target_query, None)?;
    return Ok(());
}
