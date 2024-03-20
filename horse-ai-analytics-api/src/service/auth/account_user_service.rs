use actix_web::http::header::HeaderMap;
use async_graphql::*;
use mongodb::bson::doc;
use uuid::Uuid;

use crate::graphql_object::horse_enum::ErrorType;
use crate::graphql_object::horse_model;
use crate::repository::account_users_repository;
use crate::service::jwt_service;
use crate::struct_const_def::{common_struct, db_model};

// リクエストのAuthorizationヘッダーを複合化してユーザidを取得
pub fn get_token_from_authorization_header(
    headers: &HeaderMap,
    jwt_secret: String,
) -> Option<common_struct::AuthContext> {
    match headers
        .get(actix_web::http::header::AUTHORIZATION)?
        .to_str()
    {
        Ok(auth_header) => auth_header.strip_prefix("Bearer ").and_then(|t| {
            match jwt_service::decode_jwt(t, &jwt_secret) {
                Ok(claim) => Some(common_struct::AuthContext {
                    account_id: claim.claims.contents,
                }),
                Err(_) => None,
            }
        }),
        Err(_) => None,
    }
}

// ユーザの追加
pub async fn add_account_user_by_google_auth_token(
    context: &mut &common_struct::CommonContext,
    auth_token: String,
    user_setting_id: String,
    name: String,
) -> Result<horse_model::AccountUserResponse> {
    // tokenを複合化してgmailを取得
    let decode_result = match context.secrets.get("JWT_SECRET") {
        Some(jwt_secret) => {
            let result = jwt_service::decode_jwt(&auth_token, &jwt_secret);
            result
        }
        None => {
            return Err(Error::new("Get jwt config Error")
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
    };
    let gmail = match decode_result {
        Ok(claim) => claim.claims.contents,
        Err(error) => {
            return Err(Error::new(error.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::AuthError)))
        }
    };

    // すでに登録されているものかチェック
    let find_result = account_users_repository::find_one_user_by_filter(
        context.mongo_db.clone(),
        doc! {"$or": vec! [doc! { "gmail": gmail.clone()}, doc! { "user_setting_id": user_setting_id.clone()}]},
    )
    .await;
    // 未登録であれば登録するオブジェクトを生成
    let account_user = match find_result {
        Some(_) => {
            return Err(Error::new("Already registered user")
                .extend_with(|_, e| e.set("type", ErrorType::AlreadyExistsError)))
        }
        None => db_model::AccountUsers {
            id: Uuid::new_v4().to_string(),
            user_setting_id: user_setting_id.clone(),
            name,
            gmail: Some(gmail.clone()),
            email: None,
            password: None,
        },
    };

    // 登録実行
    let register_result =
        account_users_repository::add_user(context.mongo_db.clone(), account_user.clone()).await;
    match (register_result, context.secrets.get("JWT_SECRET")) {
        (Ok(_), Some(jwt_secret)) => {
            // account_idをトークンにして返す
            let auth_token = jwt_service::make_jwt(
                &jwt_secret,
                &account_user.id,
                jwt_service::STORE_TOKEN_EXP_HOURS,
            );
            return Ok(horse_model::AccountUserResponse {
                auth_token: Some(auth_token),
                user_setting_id: account_user.user_setting_id,
                name: account_user.name,
            });
        }
        (_, _) => {
            return Err(Error::new("Can not register user")
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
    };
}

// idによるユーザ取得
pub async fn get_account_user_by_id(
    context: &mut &common_struct::CommonContext,
    account_id: String,
) -> Result<horse_model::AccountUserResponse> {
    // tokenを複合化してgmailを取得
    let find_result = account_users_repository::find_one_user_by_filter(
        context.mongo_db.clone(),
        doc! { "_id": account_id},
    )
    .await;
    match find_result {
        Some(user) => {
            return Ok(horse_model::AccountUserResponse {
                auth_token: None,
                user_setting_id: user.user_setting_id,
                name: user.name,
            });
        }
        None => {
            return Err(Error::new("User Not found")
                .extend_with(|_, e| e.set("type", ErrorType::AuthError)))
        }
    }
}
