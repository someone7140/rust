use async_graphql::*;
use chrono::prelude::*;
use futures_util::StreamExt;
use mongodb::bson;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::graphql_object::horse_enum::ErrorType;
use crate::graphql_object::horse_model;
use crate::repository::race_info_repository;
use crate::service::common_service;
use crate::service::external_info::{
    external_info_common_service, tospo_keiba_service, umanity_service,
};
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
            evaluation: memo.evaluation,
            category_id: memo.category_id.clone(),
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

// レース情報の編集
pub async fn edit_race_info(
    context: &mut &common_struct::CommonContext,
    account_user_id: String,
    input: horse_model::EditRaceInfoInputObject,
) -> Result<bool> {
    // 登録してあるデータを取得
    let mut race_info_details = race_info_repository::get_race_info_details(
        context.mongo_db.clone(),
        account_user_id.clone(),
        Some(input.id.clone()),
        None,
    )
    .await;
    let mut registered_race_info_opt = None;
    if let Some(doc) = race_info_details.next().await {
        if let Ok(race_info) = doc {
            registered_race_info_opt = Some(race_info)
        }
    }
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
                    evaluation: memo.evaluation,
                    category_id: memo.category_id.clone(),
                }
            } else {
                db_model::RaceInfoMemo {
                    id: Uuid::new_v4().to_string(),
                    title: memo.title.clone(),
                    contents: memo.contents.clone(),
                    create_date: now_date,
                    evaluation: memo.evaluation,
                    category_id: memo.category_id.clone(),
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
    race_info_id_opt: Option<String>,
) -> Result<Option<horse_model::RaceInfoDetail>> {
    // 詳細を取得クエリ実行
    let mut race_info_details = race_info_repository::get_race_info_details(
        context.mongo_db.clone(),
        account_user_id.clone(),
        race_info_id_opt,
        None,
    )
    .await;
    let mut race_info_opt = None;
    if let Some(doc) = race_info_details.next().await {
        if let Ok(race_info) = doc {
            race_info_opt = Some(race_info)
        }
    }
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
                    odds_info =
                        tospo_keiba_service::get_tospo_odds_info_from_umanity_code(&race_coe).await;
                }
            }
            _ => {}
        }
    }

    Ok(Some(get_detail_response(race_info, odds_info)))
}

// 日付指定してレース情報の詳細取得
pub async fn get_race_info_details_by_date(
    context: &mut &common_struct::CommonContext,
    account_user_id: String,
    race_info_date: String,
) -> Result<Vec<horse_model::RaceInfoDetail>> {
    // 詳細を取得クエリ実行
    let mut race_info_details = race_info_repository::get_race_info_details(
        context.mongo_db.clone(),
        account_user_id.clone(),
        None,
        Some(race_info_date),
    )
    .await;

    let mut result_vec: Vec<horse_model::RaceInfoDetail> = Vec::new();
    while let Some(doc) = race_info_details.next().await {
        if let Ok(race_info) = doc {
            result_vec.push(get_detail_response(race_info, None))
        }
    }

    Ok(result_vec)
}

// レース回答の評価値集計
pub async fn get_race_evaluation(
    context: &mut &common_struct::CommonContext,
    account_user_id: String,
    start_race_date_filter_opt: Option<String>,
    end_race_date_filter_opt: Option<String>,
) -> Result<Vec<horse_model::RaceEvaluationResult>> {
    // 日付のparse
    let utc_start_race_date = if let Some(start_race_date_filter) = start_race_date_filter_opt {
        let start_race_date =
            match common_service::get_utc_date_from_date_str(&start_race_date_filter) {
                Ok(date) => Some(date),
                Err(_) => None,
            };
        start_race_date
    } else {
        None
    };
    let utc_end_race_date = if let Some(end_race_date_filter) = end_race_date_filter_opt {
        let end_race_date = match common_service::get_utc_date_from_date_str(&end_race_date_filter)
        {
            Ok(date) => Some(date),
            Err(_) => None,
        };
        end_race_date
    } else {
        None
    };

    // DBでの集計結果の取得
    let total_results = race_info_repository::get_race_evaluation_aggregate(
        context.mongo_db.clone(),
        account_user_id.clone(),
        utc_start_race_date,
        utc_end_race_date,
        false,
    )
    .await;
    let category_results = race_info_repository::get_race_evaluation_aggregate(
        context.mongo_db.clone(),
        account_user_id.clone(),
        utc_start_race_date,
        utc_end_race_date,
        true,
    )
    .await;

    let mut evaluation_vec: Vec<horse_model::RaceEvaluationResult> = Vec::new();
    match (total_results, category_results) {
        (Ok(total_evaluate_vec), Ok(category_evaluate_vec)) => {
            for evaluate in total_evaluate_vec {
                // トータルの集計結果のタイトルをキーにカテゴリーの集計結果を取得
                let category_evaluation_list = category_evaluate_vec
                    .iter()
                    .filter(|category_evaluation| {
                        category_evaluation.key.title == evaluate.clone().key.title
                            && category_evaluation.key.category_id.is_some()
                    })
                    .map(|category_evaluation| horse_model::CategoryEvaluation {
                        category_id: category_evaluation.clone().key.category_id.unwrap(),
                        average: get_round_evaluate_value(category_evaluation.avg),
                        median: get_round_evaluate_value(category_evaluation.median),
                        count: category_evaluation.count,
                    })
                    .collect::<Vec<horse_model::CategoryEvaluation>>();
                evaluation_vec.push(horse_model::RaceEvaluationResult {
                    title: evaluate.key.title,
                    average: get_round_evaluate_value(evaluate.avg),
                    median: get_round_evaluate_value(evaluate.median),
                    count: evaluate.count,
                    category_evaluation_list,
                });
            }
        }
        (Err(e1), Err(_)) => {
            return Err(Error::new(e1.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
        (_, _) => {
            return Err(Error::new("Error aggregate evaluation")
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
    }

    Ok(evaluation_vec)
}

// 評価値の四捨五入結果を取得
fn get_round_evaluate_value(f_value: f32) -> Option<String> {
    Decimal::from_f32(f_value).map(|d| d.round_dp(2).to_string())
}

// 詳細情報のレスポンスを取得
fn get_detail_response(
    race_info: db_model::RaceInfo,
    odds_info: Option<horse_model::OddsInfoResponse>,
) -> horse_model::RaceInfoDetail {
    return horse_model::RaceInfoDetail {
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
                evaluation: memo.evaluation,
                category_id: memo.category_id.clone(),
            })
            .collect(),
    };
}
