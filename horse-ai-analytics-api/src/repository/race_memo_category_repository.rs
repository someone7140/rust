use crate::struct_const_def::db_model;
use async_graphql::futures_util::TryStreamExt;
use mongodb::{
    bson::{self, doc},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Database,
};

pub async fn add_race_memo_category(
    db: Database,
    race_memo_category: db_model::RaceMemoCategory,
) -> Result<InsertOneResult, Error> {
    let collection =
        db.collection::<db_model::RaceMemoCategory>(db_model::RACE_MEMO_CATEGORY_COLLECTION);
    let insert_one_result = collection.insert_one(race_memo_category, None).await;
    return insert_one_result;
}

pub async fn update_race_memo_category(
    db: Database,
    race_memo_category: db_model::RaceMemoCategory,
) -> Result<UpdateResult, Error> {
    let collection =
        db.collection::<db_model::RaceMemoCategory>(db_model::RACE_MEMO_CATEGORY_COLLECTION);

    let filter = doc! { "$and": [
        doc! { "account_user_id": race_memo_category.account_user_id.clone()},
        doc! { "_id": race_memo_category.id.clone()},
    ]};
    let update_result = collection
        .replace_one(filter, race_memo_category, None)
        .await;
    return update_result;
}

pub async fn delete_race_memo_category(
    db: Database,
    account_user_id: String,
    race_memo_category_id: String,
) -> Result<DeleteResult, Error> {
    let collection =
        db.collection::<db_model::RaceMemoCategory>(db_model::RACE_MEMO_CATEGORY_COLLECTION);
    // フィルター
    let filter_doc = doc! { "$and": [
        doc! { "account_user_id": account_user_id.clone()},
        doc! { "_id": race_memo_category_id.clone()},
    ]};

    let delete_one_result = collection.delete_one(filter_doc, None).await;
    return delete_one_result;
}

pub async fn get_memo_categories(
    db: Database,
    account_user_id: String,
    id_filter_opt: Option<String>,
) -> Result<Vec<db_model::RaceMemoCategory>, Error> {
    let collection =
        db.collection::<db_model::RaceMemoCategory>(db_model::RACE_MEMO_CATEGORY_COLLECTION);

    let mut pipeline = vec![
        doc! { "$match": doc! { "account_user_id": account_user_id.clone() } },
        doc! { "$addFields": doc! {"sort_field": {"$ifNull": ["$display_order", i32::MAX]}} },
        doc! { "$sort": doc! { "sort_field": 1 } },
        doc! { "$project": doc!{
            "_id": 1,
            "account_user_id": 1,
            "name": 1,
            "display_order": 1,
        }},
    ];
    // id指定あれば条件に追加
    if let Some(id_filter) = id_filter_opt {
        pipeline.insert(0, doc! { "$match": doc! { "_id": id_filter } });
    };

    let mut results: Vec<db_model::RaceMemoCategory> = Vec::new();
    let aggregate_result = collection.aggregate(pipeline, None).await;
    match aggregate_result {
        Ok(cur) => {
            let mut cur_mut = cur;
            while let Some(result) = cur_mut.try_next().await? {
                if let Ok(race_category) = bson::from_document::<db_model::RaceMemoCategory>(result)
                {
                    results.push(race_category)
                }
            }
        }
        Err(error) => return Err(error),
    }

    return Ok(results);
}
