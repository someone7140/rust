use crate::model::db::event_collection::EventCollection;
use crate::repository::mongodb_client;
use mongodb::bson::doc;
use std::error::Error;

pub fn add_events(add_event_collections: Vec<EventCollection>) -> Result<(), Box<dyn Error>> {
    let col = mongodb_client::get_mongodb_db_connection()?.collection::<EventCollection>("event");
    col.insert_many(add_event_collections, None)?;
    return Ok(());
}

pub fn delete_events(
    location_key: String,
    event_date: String,
    update_time: i64,
) -> Result<(), Box<dyn Error>> {
    let col = mongodb_client::get_mongodb_db_connection()?.collection::<EventCollection>("event");
    col.delete_many(
        doc! {
            "location_key": location_key, "event_date": event_date, "update_time": update_time
        },
        None,
    )?;
    return Ok(());
}
