use crate::{
    service::common_service,
    struct_const_def::db_model::{self, RaceInfo},
};
use mongodb::{
    bson::{self, doc},
    error::Error,
    options::FindOptions,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Cursor, Database,
};

pub async fn add_race_info(
    db: Database,
    race_info: db_model::RaceInfo,
) -> Result<InsertOneResult, Error> {
    let collection = db.collection::<db_model::RaceInfo>(db_model::RACE_INFO_COLLECTION);
    let insert_one_result = collection.insert_one(race_info, None).await;
    return insert_one_result;
}

pub async fn update_race_info(
    db: Database,
    race_info: db_model::RaceInfo,
) -> Result<UpdateResult, Error> {
    let collection = db.collection::<db_model::RaceInfo>(db_model::RACE_INFO_COLLECTION);

    let filter = doc! { "$and": [
        doc! { "account_user_id": race_info.account_user_id.clone()},
        doc! { "_id": race_info.id.clone()},
    ]};
    let update_result = collection.replace_one(filter, race_info, None).await;
    return update_result;
}

pub async fn delete_race_info(
    db: Database,
    account_user_id: String,
    race_info_id: String,
) -> Result<DeleteResult, Error> {
    let collection = db.collection::<db_model::RaceInfo>(db_model::RACE_INFO_COLLECTION);
    // フィルター
    let filter_doc = doc! { "$and": [
        doc! { "account_user_id": account_user_id.clone()},
        doc! { "_id": race_info_id.clone()},
    ]};

    let delete_one_result = collection.delete_one(filter_doc, None).await;
    return delete_one_result;
}

pub async fn get_race_info_list(
    db: Database,
    account_user_id: String,
    start_race_date_opt: Option<String>,
    end_race_date_str_opt: Option<String>,
    keyowrd_opt: Option<String>,
    limit: i64,
) -> Cursor<RaceInfo> {
    let collection = db.collection::<db_model::RaceInfo>(db_model::RACE_INFO_COLLECTION);

    // フィルター
    let mut filter_doc = doc! { "account_user_id": account_user_id.clone()};
    // 開始日
    if let Some(start_race_date_str) = start_race_date_opt {
        if let Ok(start_race_date_utc) =
            common_service::get_utc_date_from_date_str(&start_race_date_str)
        {
            filter_doc.insert(
                "race_date",
                doc! { "$gte": bson::DateTime::from_millis(start_race_date_utc.timestamp_millis()) },
            );
        }
    };
    // 終了日
    if let Some(end_race_date_str) = end_race_date_str_opt {
        if let Ok(end_race_date_utc) =
            common_service::get_utc_date_from_date_str(&end_race_date_str)
        {
            filter_doc.insert(
                "race_date",
                doc! { "$lte": bson::DateTime::from_millis(end_race_date_utc.timestamp_millis()) },
            );
        }
    };
    // キーワード
    if let Some(keyword) = keyowrd_opt {
        filter_doc.insert("race_name", doc! { "$regex": keyword, "$options": "i"});
    };
    let find_options = FindOptions::builder()
        .sort(doc! {"race_date": -1})
        .limit(limit)
        .build();

    let result = collection.find(filter_doc, find_options).await;
    return result.unwrap();
}

pub async fn get_race_info_detail(
    db: Database,
    account_user_id: String,
    race_info_id: String,
) -> Option<RaceInfo> {
    let collection = db.collection::<db_model::RaceInfo>(db_model::RACE_INFO_COLLECTION);
    // フィルター
    let filter_doc = doc! { "$and": [
        doc! { "account_user_id": account_user_id.clone()},
        doc! { "_id": race_info_id.clone()},
    ]};
    let result = collection.find_one(filter_doc, None).await;
    return result.unwrap();
}
