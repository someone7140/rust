use async_graphql::*;
use chrono::prelude::*;
use futures_util::StreamExt;
use mongodb::bson;
use uuid::Uuid;

use crate::graphql_object::horse_enum::ErrorType;
use crate::graphql_object::horse_model::{self};
use crate::repository::race_info_repository;
use crate::service::common_service;
use crate::struct_const_def::{common_struct, db_model};

// レース情報の追加
pub async fn add_race_info(
    context: &mut &common_struct::CommonContext,
    account_user_id: String,
    input: horse_model::AddRaceInfoInputObject,
) -> Result<bool> {
    // 日付のパース
    let utc_race_date = match common_service::get_utc_date_from_date_str(&input.race_date) {
        Ok(date) => date,
        Err(error) => return Err(error),
    };
    // 登録するdbモデル
    let memo_list = input
        .memo_list
        .iter()
        .map(|memo| db_model::RaceInfoMemo {
            id: Uuid::new_v4().to_string(),
            title: memo.title.clone(),
            contents: memo.contents.clone(),
            create_date: bson::DateTime::from_millis(Utc::now().timestamp_millis()),
        })
        .collect::<Vec<db_model::RaceInfoMemo>>();
    let race_info_model = db_model::RaceInfo {
        id: Uuid::new_v4().to_string(),
        account_user_id,
        race_name: input.race_name,
        race_date: bson::DateTime::from_millis(utc_race_date.timestamp_millis()),
        analytics_url: input.analytics_url,
        prompt: input.prompt,
        memo_list,
    };

    // 登録実行
    let register_result =
        race_info_repository::add_race_info(context.mongo_db.clone(), race_info_model).await;
    return match register_result {
        Ok(_) => Ok(true),
        Err(error) => {
            return Err(Error::new(error.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
    };
}

// 登録したレース情報の一覧取得
pub async fn get_race_info_list_by_account(
    context: &mut &common_struct::CommonContext,
    account_user_id: String,
    filter: horse_model::RaceInfoListFilterInputObject,
) -> Result<Vec<horse_model::RaceInfoForList>> {
    // クエリ実行
    let mut race_info_cur = race_info_repository::get_race_info_list(
        context.mongo_db.clone(),
        account_user_id.clone(),
        filter.start_race_date,
        filter.end_race_date,
        filter.keyword,
        200,
    )
    .await;

    let mut result_vec: Vec<horse_model::RaceInfoForList> = Vec::new();
    while let Some(doc) = race_info_cur.next().await {
        if let Ok(race_info) = doc {
            result_vec.push(horse_model::RaceInfoForList {
                id: race_info.id,
                race_name: race_info.race_name,
                race_date: match common_service::get_jst_date_from_timestamp_millis(
                    race_info.race_date.timestamp_millis(),
                ) {
                    Ok(jst_date) => jst_date.format("%Y/%m/%d").to_string(),
                    _ => "".to_string(),
                },
            })
        }
    }

    return Ok(result_vec);
}
