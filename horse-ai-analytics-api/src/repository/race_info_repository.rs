use mongodb::{error::Error, results::InsertOneResult, Database};

use crate::struct_const_def::db_model;

pub async fn add_race_info(
    db: Database,
    race_info: db_model::RaceInfo,
) -> Result<InsertOneResult, Error> {
    let collection = db.collection::<db_model::RaceInfo>(db_model::RACE_INFO_COLLECTION);
    let insert_one_result = collection.insert_one(race_info, None).await;
    return insert_one_result;
}
