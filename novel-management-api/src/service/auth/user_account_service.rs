use std::collections::HashMap;

use async_graphql::*;
use entity::user_accounts;
use uuid::Uuid;

use crate::model::common::context_info;
use crate::model::graphql::graphql_error::AppError;
use crate::model::graphql::graphql_user_account;
use crate::repository::user_account_repository;
use crate::service::auth::jwt_service::{
    self, JWT_GMAIL_KEY, JWT_IMAGE_URL_KEY, JWT_USER_ACCOUNT_ID_KEY,
};
use crate::service::common::data_time_service;

// ユーザー登録を行う
pub async fn add_user_account_by_gmail(
    context: &mut &context_info::CommonContext,
    register_token: String,
    user_setting_id: String,
    name: String,
) -> Result<graphql_user_account::UserAccountResponse> {
    // tokenを複合化して登録用の情報を取得
    let (decode_result, jwt_secret) = match context.secrets.get("JWT_SECRET") {
        Some(jwt_secret) => {
            let result = jwt_service::decode_jwt(&register_token, &jwt_secret);
            (result, jwt_secret)
        }
        None => return Err(AppError::SystemError("Get jwt config Error".to_string()).extend()),
    };
    let google_info_map = match decode_result {
        Ok(claim) => claim.claims.contents,
        Err(error) => return Err(AppError::AuthorizationError(error.to_string()).extend()),
    };
    let gmail = match google_info_map.get(JWT_GMAIL_KEY) {
        Some(gmail) => gmail,
        None => return Err(AppError::SystemError("Get jwt key Error".to_string()).extend()),
    };
    let image_url = google_info_map.get(JWT_IMAGE_URL_KEY);

    // すでに登録されているものかチェック
    let find_result_user_setting_id = user_account_repository::get_user_account_by_user_setting_id(
        &context.db_connect,
        user_setting_id.clone(),
    )
    .await;
    if find_result_user_setting_id.is_some() {
        return Err(
            AppError::ForbiddenError("Already registered user_setting".to_string()).extend(),
        );
    }
    let find_result_gmail =
        user_account_repository::get_user_account_by_gmail(&context.db_connect, gmail.to_string())
            .await;
    if find_result_gmail.is_some() {
        return Err(AppError::ForbiddenError("Already registered gmail".to_string()).extend());
    }

    // 未登録であれば登録
    let new_user_account = user_accounts::Model {
        id: Uuid::now_v7().to_string(),
        user_setting_id: user_setting_id.clone(),
        gmail: gmail.clone(),
        image_url: image_url.cloned(),
        created_at: data_time_service::get_now_jst_datetime_fixed_offset().into(),
    };
    let register_error = user_account_repository::register_user_account(
        &context.db_connect,
        new_user_account.clone().into(),
    )
    .await;
    if register_error.is_some() {
        return Err(AppError::SystemError(register_error.unwrap().to_string()).extend());
    }

    // ユーザーのIDからトークンを生成
    let auth_token = jwt_service::make_jwt(
        &jwt_secret,
        HashMap::from([(JWT_USER_ACCOUNT_ID_KEY.to_string(), new_user_account.id)]),
        jwt_service::STORE_TOKEN_EXP_HOURS,
    );
    Ok(graphql_user_account::UserAccountResponse {
        token: auth_token,
        user_setting_id: user_setting_id,
        name: name,
        image_url: image_url.cloned(),
    })
}
