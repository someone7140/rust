use async_graphql::*;
use chrono::prelude::*;
use futures_util::StreamExt;
use mongodb::bson;
use uuid::Uuid;

use crate::graphql_object::horse_enum::ErrorType;
use crate::graphql_object::horse_model::{self};
use crate::repository::race_info_repository;
use crate::service::common_service;
use crate::service::external_info::{external_info_common_service, umanity_service};
use crate::struct_const_def::{common_struct, db_model};

// レース情報の追加
pub async fn add_race_info(
    context: &mut &common_struct::CommonContext,
    account_user_id: String,
    input: horse_model::RaceInfoInputObject,
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

// レース情報の追加
pub async fn edit_race_info(
    context: &mut &common_struct::CommonContext,
    account_user_id: String,
    input: horse_model::EditRaceInfoInputObject,
) -> Result<bool> {
    // 登録してあるデータを取得
    let registered_race_info_opt = race_info_repository::get_race_info_detail(
        context.mongo_db.clone(),
        account_user_id.clone(),
        input.id.clone(),
    )
    .await;
    let registered_race_info = match registered_race_info_opt {
        Some(race_info) => race_info,
        None => {
            return Err(Error::new("Can not get race info")
                .extend_with(|_, e| e.set("type", ErrorType::BadRequest)))
        }
    };

    // 日付のパース
    let utc_race_date = match common_service::get_utc_date_from_date_str(&input.race_info.race_date)
    {
        Ok(date) => date,
        Err(error) => return Err(error),
    };
    // 登録するdbモデル
    let memo_list = input
        .race_info
        .memo_list
        .iter()
        .map(|memo| {
            let now_date = bson::DateTime::from_millis(Utc::now().timestamp_millis());
            if let Some(id) = &(memo.id) {
                // 登録されたメモがあるか
                let memo_registered_opt = registered_race_info
                    .memo_list
                    .iter()
                    .find(|registered_race_memo| id.to_string() == registered_race_memo.id);

                db_model::RaceInfoMemo {
                    id: id.to_string(),
                    title: memo.title.clone(),
                    contents: memo.contents.clone(),
                    create_date: if let Some(memo_registered) = memo_registered_opt {
                        memo_registered.create_date
                    } else {
                        now_date
                    },
                }
            } else {
                db_model::RaceInfoMemo {
                    id: Uuid::new_v4().to_string(),
                    title: memo.title.clone(),
                    contents: memo.contents.clone(),
                    create_date: now_date,
                }
            }
        })
        .collect::<Vec<db_model::RaceInfoMemo>>();

    let race_info_model = db_model::RaceInfo {
        id: input.id,
        account_user_id: account_user_id.clone(),
        race_name: input.race_info.race_name,
        race_date: bson::DateTime::from_millis(utc_race_date.timestamp_millis()),
        analytics_url: input.race_info.analytics_url,
        prompt: input.race_info.prompt,
        memo_list,
    };

    // 登録実行
    let update_result =
        race_info_repository::update_race_info(context.mongo_db.clone(), race_info_model).await;
    return match update_result {
        Ok(_) => Ok(true),
        Err(error) => {
            return Err(Error::new(error.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
    };
}

// レース情報の削除
pub async fn delete_race_info(
    context: &mut &common_struct::CommonContext,
    account_user_id: String,
    race_info_id: String,
) -> Result<bool> {
    // 削除実行
    let delete_result = race_info_repository::delete_race_info(
        context.mongo_db.clone(),
        account_user_id.clone(),
        race_info_id.clone(),
    )
    .await;
    return match delete_result {
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
    filter_opt: Option<horse_model::RaceInfoListFilterInputObject>,
) -> Result<Vec<horse_model::RaceInfoForList>> {
    let limit: i64 = 200;
    // クエリ実行
    let mut race_info_cur = match filter_opt {
        Some(filter) => {
            race_info_repository::get_race_info_list(
                context.mongo_db.clone(),
                account_user_id.clone(),
                filter.start_race_date,
                filter.end_race_date,
                filter.keyword,
                limit,
            )
            .await
        }
        None => {
            race_info_repository::get_race_info_list(
                context.mongo_db.clone(),
                account_user_id.clone(),
                None,
                None,
                None,
                limit,
            )
            .await
        }
    };

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

// 指定したレース情報の詳細取得
pub async fn get_race_info_detail(
    context: &mut &common_struct::CommonContext,
    account_user_id: String,
    race_info_id: String,
) -> Result<Option<horse_model::RaceInfoDetail>> {
    // 詳細を取得クエリ実行
    let race_info_opt = race_info_repository::get_race_info_detail(
        context.mongo_db.clone(),
        account_user_id.clone(),
        race_info_id.clone(),
    )
    .await;

    let race_info = match race_info_opt {
        Some(race_info) => race_info,
        None => return Ok(None),
    };

    // オッズの取得
    let mut odds_info: Option<horse_model::OddsInfoResponse> = None;
    if let Some(analytics_url) = &(race_info.analytics_url) {
        // 指定されたウマニティのページによってオッズの取得を行う
        match (
            external_info_common_service::get_page_from_url(&analytics_url),
            umanity_service::get_race_code_and_date_from_url_code(&analytics_url),
        ) {
            (Ok(page), Ok((race_coe, _))) => {
                if page != "race_7.php" {
                    odds_info = umanity_service::get_odds_info_from_race_8_9(&race_coe).await;
                }
            }
            _ => {}
        }
    }

    let response = horse_model::RaceInfoDetail {
        id: race_info.id,
        race_name: race_info.race_name,
        analytics_url: race_info.analytics_url,
        race_date: match common_service::get_jst_date_from_timestamp_millis(
            race_info.race_date.timestamp_millis(),
        ) {
            Ok(jst_date) => jst_date.format("%Y/%m/%d").to_string(),
            _ => "".to_string(),
        },
        prompt: race_info.prompt,
        odds: odds_info,
        memo_list: race_info
            .memo_list
            .iter()
            .map(|memo| horse_model::RaceMemo {
                id: memo.id.clone(),
                title: memo.title.clone(),
                contents: memo.contents.clone(),
            })
            .collect(),
    };
    Ok(Some(response))
}
