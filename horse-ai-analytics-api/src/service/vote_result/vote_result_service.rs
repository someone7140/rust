use async_graphql::*;
use chrono::DateTime;
use chrono_tz::Tz;
use mongodb::bson;
use uuid::Uuid;

use crate::graphql_object::horse_enum::ErrorType;
use crate::graphql_object::horse_model;
use crate::repository::vote_result_repository;
use crate::service::common_service;
use crate::struct_const_def::{common_struct, db_model};

// 投票内容の追加
pub async fn add_vote_info(
    context: &mut &common_struct::CommonContext,
    account_user_id: String,
    input: horse_model::VoteResultInputObject,
) -> Result<bool> {
    // 日付のパース
    let utc_race_date = match common_service::get_utc_date_from_date_str(&input.race_date) {
        Ok(date) => date,
        Err(error) => return Err(error),
    };
    // 登録実行
    let result = vote_result_repository::add_vote_result(
        context.mongo_db.clone(),
        convert_input_to_db_model(input, account_user_id, utc_race_date),
    )
    .await;
    return match result {
        Ok(_) => Ok(true),
        Err(error) => {
            return Err(Error::new(error.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
    };
}

// 投票内容の入力からDBモデルに変換
fn convert_input_to_db_model(
    input: horse_model::VoteResultInputObject,
    account_user_id: String,
    utc_race_date: DateTime<Tz>,
) -> db_model::VoteResult {
    let race_info_list = input
        .vote_race_list
        .iter()
        .map(|r| db_model::VoteRace {
            race_id: r.race_id.to_string(),
            vote_race_contents: r
                .vote_race_contents
                .iter()
                .map(|c| db_model::VoteRaceContent {
                    id: c.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string()),
                    most_priority_memo_id: c.most_priority_memo_id.clone(),
                    contents: c.contents.clone(),
                    bet_amount: c.bet_amount,
                    return_amount: c.return_amount,
                })
                .collect::<Vec<db_model::VoteRaceContent>>(),
        })
        .collect::<Vec<db_model::VoteRace>>();

    db_model::VoteResult {
        id: input.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
        account_user_id,
        race_date: bson::DateTime::from_millis(utc_race_date.timestamp_millis()),
        vote_race_list: race_info_list,
    }
}
