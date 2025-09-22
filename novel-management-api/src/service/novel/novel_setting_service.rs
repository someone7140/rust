use async_graphql::*;
use entity::novel_settings;
use uuid::Uuid;

use crate::model::common::context_info;
use crate::model::graphql::graphql_error::AppError;
use crate::model::graphql::graphql_novel_setting;
use crate::repository::novel_setting_repository;

// 小説の設定一覧を取得
pub async fn get_novel_settings(
    context: &mut &context_info::CommonContext,
    user_account_id: String,
    novel_id: String,
) -> Result<Vec<graphql_novel_setting::NovelSettingResponse>> {
    let find_results = novel_setting_repository::get_novel_settings(
        &context.db_connect,
        user_account_id,
        novel_id,
    )
    .await;

    Ok(find_results
        .iter()
        .map(|setting| graphql_novel_setting::NovelSettingResponse {
            id: setting.id.clone(),
            name: setting.name.clone(),
            novel_id: setting.novel_id.clone(),
            parent_setting_id: setting.parent_setting_id.clone(),
            display_order: setting.display_order,
            attributes: setting.attributes.clone(),
            description: setting.description.clone(),
        })
        .collect::<Vec<graphql_novel_setting::NovelSettingResponse>>())
}

// 親設定をキーにした設定一覧を取得
pub async fn get_novel_settings_by_parent_setting_id(
    context: &mut &context_info::CommonContext,
    user_account_id: String,
    parent_setting_id: String,
) -> Result<Vec<graphql_novel_setting::NovelSettingResponse>> {
    let find_results = novel_setting_repository::get_novel_settings_by_parent_setting_id(
        &context.db_connect,
        user_account_id,
        parent_setting_id,
    )
    .await;

    Ok(find_results
        .iter()
        .map(|setting| graphql_novel_setting::NovelSettingResponse {
            id: setting.id.clone(),
            name: setting.name.clone(),
            novel_id: setting.novel_id.clone(),
            parent_setting_id: setting.parent_setting_id.clone(),
            display_order: setting.display_order,
            attributes: setting.attributes.clone(),
            description: setting.description.clone(),
        })
        .collect::<Vec<graphql_novel_setting::NovelSettingResponse>>())
}

// 設定を登録
pub async fn register_novel_settings(
    context: &mut &context_info::CommonContext,
    user_account_id: String,
    inputs: Vec<graphql_novel_setting::NovelSettingRegisterInput>,
) -> Result<bool> {
    // 新規追加
    let add_settings = inputs
        .iter()
        .filter(|input| input.id.is_none())
        .map(|input| novel_settings::Model {
            id: Uuid::now_v7().to_string(),
            name: input.name.clone(),
            novel_id: input.novel_id.clone(),
            parent_setting_id: input.parent_setting_id.clone(),
            owner_user_account_id: user_account_id.clone(),
            display_order: input.display_order,
            attributes: input.attributes.clone(),
            description: input.description.clone(),
        })
        .map(novel_settings::ActiveModel::from)
        .collect::<Vec<novel_settings::ActiveModel>>();
    if add_settings.len() > 0 {
        if let Some(error) =
            novel_setting_repository::add_settings(&context.db_connect, add_settings).await
        {
            return Err(async_graphql::Error::new(error.to_string()));
        }
    }

    // 更新
    let update_settings = inputs
        .iter()
        .filter(|input| input.id.is_some())
        .map(|input| novel_settings::Model {
            id: input.id.clone().unwrap(),
            name: input.name.clone(),
            novel_id: input.novel_id.clone(),
            parent_setting_id: input.parent_setting_id.clone(),
            owner_user_account_id: user_account_id.clone(),
            display_order: input.display_order,
            attributes: input.attributes.clone(),
            description: input.description.clone(),
        })
        .collect::<Vec<novel_settings::Model>>();
    if update_settings.len() > 0 {
        if let Some(error) =
            novel_setting_repository::update_settings(&context.db_connect, update_settings).await
        {
            return Err(async_graphql::Error::new(error.to_string()));
        }
    }

    Ok(true)
}

// 設定のIDを指定キーで削除
pub async fn delete_novel_settings_by_id(
    context: &mut &context_info::CommonContext,
    user_account_id: String,
    setting_id: String,
) -> Result<bool> {
    let find_result = novel_setting_repository::get_novel_setting_by_setting_id(
        &context.db_connect,
        user_account_id.clone(),
        setting_id,
    )
    .await;

    let novel_setting = match find_result {
        Some(setting) => setting,
        None => return Ok(true),
    };

    if let Some(parent_setting_id) = &novel_setting.parent_setting_id {
        if let Some(error) = novel_setting_repository::delete_settings_by_parent_setting_id(
            &context.db_connect,
            user_account_id.clone(),
            parent_setting_id.clone(),
        )
        .await
        {
            return Err(async_graphql::Error::new(error.to_string()));
        }
    }

    let delete_error =
        novel_setting_repository::delete_novel_setting(&context.db_connect, novel_setting.into())
            .await;
    if delete_error.is_some() {
        return Err(AppError::SystemError(delete_error.unwrap().to_string()).extend());
    }

    Ok(true)
}

// 設定のIDを複数指定キーで削除
pub async fn delete_novel_settings_by_ids(
    context: &mut &context_info::CommonContext,
    user_account_id: String,
    setting_ids: Vec<String>,
) -> Result<bool> {
    if let Some(error) = novel_setting_repository::delete_settings_by_ids(
        &context.db_connect,
        user_account_id.clone(),
        setting_ids,
    )
    .await
    {
        return Err(async_graphql::Error::new(error.to_string()));
    }

    Ok(true)
}
