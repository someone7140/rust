use crate::model::db::search_condition_collection::{LocationCollection, StoreTypeCollection};
use crate::repository::mongodb_client;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use std::error::Error;

pub fn get_location_list() -> Result<Vec<LocationCollection>, Box<dyn Error>> {
    let col =
        mongodb_client::get_mongodb_db_connection()?.collection::<LocationCollection>("location");
    let find_options = FindOptions::builder()
        .sort(doc! { "sort_order": 1 })
        .build();
    let results = col.find(None, find_options)?.flatten().collect();
    return Ok(results);
}

pub fn get_store_type_list() -> Result<Vec<StoreTypeCollection>, Box<dyn Error>> {
    let col = mongodb_client::get_mongodb_db_connection()?
        .collection::<StoreTypeCollection>("store_type");
    let find_options = FindOptions::builder()
        .sort(doc! { "sort_order": 1 })
        .build();
    let results = col.find(None, find_options)?.flatten().collect();
    return Ok(results);
}
