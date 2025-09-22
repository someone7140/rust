use async_graphql::*;
use entity::novels;
use uuid::Uuid;

use crate::model::common::context_info;
use crate::model::graphql::graphql_error::AppError;
use crate::model::graphql::graphql_novel;
use crate::repository::{novel_repository, novel_setting_repository};
use crate::service::common::data_time_service;

// 小説を追加
pub async fn add_novel(
    context: &mut &context_info::CommonContext,
    user_account_id: String,
    title: String,
    description: Option<String>,
) -> Result<bool> {
    let new_novel = novels::Model {
        id: Uuid::now_v7().to_string(),
        title: title,
        description: description,
        owner_user_account_id: user_account_id,
        created_at: data_time_service::get_now_jst_datetime_fixed_offset().into(),
    };
    let register_error =
        novel_repository::register_novel(&context.db_connect, new_novel.into()).await;
    if register_error.is_some() {
        return Err(AppError::SystemError(register_error.unwrap().to_string()).extend());
    }

    Ok(true)
}

// 小説を編集
pub async fn edit_novel(
    context: &mut &context_info::CommonContext,
    user_account_id: String,
    novel_id: String,
    title: String,
    description: Option<String>,
) -> Result<bool> {
    let find_result =
        novel_repository::get_novel_by_id(&context.db_connect, user_account_id, novel_id).await;
    let novel: novels::Model = match find_result {
        Some(novel) => novel,
        None => return Err(AppError::NotFoundError("Can not find novel".to_string()).extend()),
    };

    let edit_error =
        novel_repository::edit_novel(&context.db_connect, novel.into(), title, description).await;
    if edit_error.is_some() {
        return Err(AppError::SystemError(edit_error.unwrap().to_string()).extend());
    }

    Ok(true)
}

// 小説を削除
pub async fn delete_novel(
    context: &mut &context_info::CommonContext,
    user_account_id: String,
    novel_id: String,
) -> Result<bool> {
    let find_result = novel_repository::get_novel_by_id(
        &context.db_connect,
        user_account_id.clone(),
        novel_id.clone(),
    )
    .await;
    let novel = match find_result {
        Some(novel) => novel,
        None => return Ok(true),
    };

    // 設定を削除
    if let Some(error) = novel_setting_repository::delete_settings_by_novel_id(
        &context.db_connect,
        user_account_id.clone(),
        novel_id,
    )
    .await
    {
        return Err(async_graphql::Error::new(error.to_string()));
    }

    let delete_error =
        novel_repository::delete_novel(&context.db_connect, novel.into(), user_account_id).await;
    if delete_error.is_some() {
        return Err(AppError::SystemError(delete_error.unwrap().to_string()).extend());
    }

    Ok(true)
}

// 小説の一覧を取得
pub async fn get_my_novels(
    context: &mut &context_info::CommonContext,
    user_account_id: String,
) -> Result<Vec<graphql_novel::NovelResponse>> {
    let find_results = novel_repository::get_my_novels(&context.db_connect, user_account_id).await;

    Ok(find_results
        .iter()
        .map(|n| graphql_novel::NovelResponse {
            id: n.id.clone(),
            title: n.title.clone(),
            description: n.description.clone(),
        })
        .collect::<Vec<graphql_novel::NovelResponse>>())
}
