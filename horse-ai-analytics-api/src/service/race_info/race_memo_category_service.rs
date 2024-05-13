use async_graphql::*;
use uuid::Uuid;

use crate::graphql_object::horse_enum::ErrorType;
use crate::graphql_object::horse_model;
use crate::repository::{race_info_repository, race_memo_category_repository};

use crate::struct_const_def::{common_struct, db_model};

// メモのカテゴリーの追加
pub async fn add_race_memo_category(
    context: &mut &common_struct::CommonContext,
    account_user_id: String,
    name: String,
    display_order: Option<i32>,
) -> Result<bool> {
    // 登録するdbモデル
    let memo_category = db_model::RaceMemoCategory {
        id: Uuid::new_v4().to_string(),
        account_user_id,
        name,
        display_order,
    };

    // 登録実行
    let register_result = race_memo_category_repository::add_race_memo_category(
        context.mongo_db.clone(),
        memo_category,
    )
    .await;

    return match register_result {
        Ok(_) => Ok(true),
        Err(error) => {
            return Err(Error::new(error.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
    };
}

// メモのカテゴリーの編集
pub async fn edit_race_memo_category(
    context: &mut &common_struct::CommonContext,
    account_user_id: String,
    category_id: String,
    name: String,
    display_order: Option<i32>,
) -> Result<bool> {
    // 登録してあるデータを取得
    let registered_results = race_memo_category_repository::get_memo_categories(
        context.mongo_db.clone(),
        account_user_id.clone(),
        Some(category_id.clone()),
    )
    .await;
    match registered_results {
        Ok(registered_memo_categories) => {
            if registered_memo_categories.len() < 1 {
                return Err(Error::new("Can not get memo category")
                    .extend_with(|_, e| e.set("type", ErrorType::BadRequest)));
            }
        }
        Err(error) => {
            return Err(Error::new(error.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
    };

    // 登録するdbモデル
    let memo_category = db_model::RaceMemoCategory {
        id: category_id,
        account_user_id,
        name,
        display_order,
    };

    // 編集実行
    let edit_result = race_memo_category_repository::update_race_memo_category(
        context.mongo_db.clone(),
        memo_category,
    )
    .await;

    return match edit_result {
        Ok(_) => Ok(true),
        Err(error) => {
            return Err(Error::new(error.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
    };
}

// 登録したメモカテゴリーの一覧取得
pub async fn get_race_memo_category_list_by_account(
    context: &mut &common_struct::CommonContext,
    account_user_id: String,
    id_filter: Option<String>,
) -> Result<Vec<horse_model::RaceMemoCategory>> {
    // クエリ実行
    let race_memo_categories_result = race_memo_category_repository::get_memo_categories(
        context.mongo_db.clone(),
        account_user_id.clone(),
        id_filter,
    )
    .await;

    let mut result_vec: Vec<horse_model::RaceMemoCategory> = Vec::new();
    match race_memo_categories_result {
        Ok(race_memo_categories) => {
            for race_memo_category in race_memo_categories {
                result_vec.push(horse_model::RaceMemoCategory {
                    id: race_memo_category.id,
                    name: race_memo_category.name,
                    display_order: race_memo_category.display_order,
                })
            }
        }
        Err(error) => {
            return Err(Error::new(error.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
    }

    return Ok(result_vec);
}

// メモカテゴリーの削除
pub async fn delete_race_memo_category(
    context: &mut &common_struct::CommonContext,
    account_user_id: String,
    category_id: String,
) -> Result<bool> {
    // カテゴリーが紐づくメモの紐付けを解除
    let detach_result = race_info_repository::detach_race_memo_category(
        context.mongo_db.clone(),
        account_user_id.clone(),
        category_id.clone(),
    )
    .await;
    if let Err(error) = detach_result {
        return Err(
            Error::new(error.to_string()).extend_with(|_, e| e.set("type", ErrorType::SystemError))
        );
    };

    // 削除実行
    let delete_result = race_memo_category_repository::delete_race_memo_category(
        context.mongo_db.clone(),
        account_user_id.clone(),
        category_id.clone(),
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
