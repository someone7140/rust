use crate::{service::common_service, struct_const_def::db_model};
use async_graphql::futures_util::TryStreamExt;
use chrono::DateTime;
use chrono_tz::Tz;
use mongodb::{
    bson::{self, doc, Bson},
    error::Error,
    options::{FindOptions, UpdateOptions},
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
) -> Cursor<db_model::RaceInfo> {
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
) -> Option<db_model::RaceInfo> {
    let collection = db.collection::<db_model::RaceInfo>(db_model::RACE_INFO_COLLECTION);
    // フィルター
    let filter_doc = doc! { "$and": [
        doc! { "account_user_id": account_user_id.clone()},
        doc! { "_id": race_info_id.clone()},
    ]};
    let result = collection.find_one(filter_doc, None).await;
    return result.unwrap();
}

pub async fn get_race_evaluation_aggregate(
    db: Database,
    account_user_id: String,
    start_race_date_time_opt: Option<DateTime<Tz>>,
    end_race_date_time_opt: Option<DateTime<Tz>>,
) -> Result<Vec<db_model::RaceEvaluationAggregate>, Error> {
    let collection = db.collection::<db_model::RaceInfo>(db_model::RACE_INFO_COLLECTION);

    let mut pipeline = vec![
        doc! { "$match": doc! { "account_user_id": account_user_id.clone() } },
        doc! { "$unwind": "$memo_list" },
        doc! { "$match": doc! { "memo_list.evaluation": { "$exists": true} } },
        doc! { "$match": doc! { "memo_list.title": { "$exists": true} } },
        doc! { "$group": doc! {
            "_id": doc! { "title": "$memo_list.title" },
            "avg": doc! { "$avg": "$memo_list.evaluation" },
            "median": doc! { "$median": doc! {  "input":"$memo_list.evaluation", "method": "approximate" } },
            "count": doc! { "$sum": 1 },
        } },
        doc! { "$sort": doc! { "avg": -1, "median": -1 } },
    ];
    // 開始日
    if let Some(start_race_date) = start_race_date_time_opt {
        pipeline.insert(
            0,
            doc! { "$match": doc! {
                "race_date": doc! { "$gte": bson::DateTime::from_millis(start_race_date.timestamp_millis()) } }
            },
        );
    };
    // 終了日
    if let Some(end_race_date) = end_race_date_time_opt {
        pipeline.insert(
                0,
                doc! { "$match": doc! { 
                    "race_date": doc! { "$lte": bson::DateTime::from_millis(end_race_date.timestamp_millis()) } }
                },
            );
    };

    let mut results: Vec<db_model::RaceEvaluationAggregate> = Vec::new();
    let aggregate_result = collection.aggregate(pipeline, None).await;
    match aggregate_result {
        Ok(cur) => {
            let mut cur_mut = cur;
            while let Some(result) = cur_mut.try_next().await? {
                if let Ok(race_evaluation) =
                    bson::from_document::<db_model::RaceEvaluationAggregate>(result)
                {
                    results.push(race_evaluation)
                }
            }
        }
        Err(error) => return Err(error),
    }

    return Ok(results);
}

pub async fn detach_race_memo_category(
    db: Database,
    account_user_id: String,
    race_memo_category_id: String,
) -> Result<UpdateResult, Error> {
    let collection = db.collection::<db_model::RaceInfo>(db_model::RACE_INFO_COLLECTION);
    // ドキュメントのフィルター
    let filter_doc = doc! { "$and": [
        doc! { "account_user_id": account_user_id.clone()},
    ]};
    // 更新内容
    let set_doc = doc! {
        "$set": doc! { "memo_list.$[element].category_id": Bson::Null }
    };
    // オプションに配列のフィルターをセット
    let update_options = UpdateOptions::builder()
        .array_filters(vec![doc! { "element.category_id": race_memo_category_id }])
        .build();

    return collection
        .update_many(filter_doc, set_doc, update_options)
        .await;
}
