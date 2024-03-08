use async_graphql::*;
use google_oauth::{AsyncClient, GoogleAccessTokenPayload};
use mongodb::bson::doc;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl,
};

use crate::graphql_object::horse_enum::ErrorType;
use crate::graphql_object::horse_model;
use crate::repository::account_users_repository;
use crate::service::jwt_service;
use crate::struct_const_def::common_struct;

// googleの認可コードを検証し認証用トークンを生成
pub async fn validate_auth_code(
    context: &mut &common_struct::CommonContext,
    auth_code: String,
) -> Result<horse_model::ValidateGoogleAuthCodeResponse> {
    // 認可コードからtokenを取得
    let auth_result = match (
        context.secrets.get("GOOGLE_AUTH_CLIENT_ID"),
        context.secrets.get("GOOGLE_AUTH_CLIENT_SECRET"),
        context.secrets.get("FRONT_DOMAIN"),
    ) {
        (Some(client_id), Some(client_secret), Some(redirect_url)) => {
            get_token_from_google_auth_code(auth_code, client_id, client_secret, redirect_url).await
        }
        (_, _, _) => {
            return Err(Error::new("Get google auth config Error")
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
    };

    // googleのtokenからgmailを取得し認証用トークンを生成
    let auth_token = match (auth_result, context.secrets.get("JWT_SECRET")) {
        (Ok(google_token), Some(jwt_secret)) => {
            let gmail = google_token.email.unwrap();
            let find_result = account_users_repository::find_one_user_by_filter(
                context.mongo_db.clone(),
                doc! { "gmail": gmail.clone()},
            )
            .await;
            let token = match find_result {
                Some(_) => {
                    return Err(Error::new("Already registered user")
                        .extend_with(|_, e| e.set("type", ErrorType::AlreadyExistsError)))
                }
                None => {
                    jwt_service::make_jwt(&jwt_secret, &gmail, jwt_service::TEMP_TOKEN_EXP_HOURS)
                }
            };
            token
        }
        (Err(error), _) => return Err(error),
        (_, _) => {
            return Err(Error::new("Failed google auth")
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
    };
    Ok(horse_model::ValidateGoogleAuthCodeResponse { auth_token })
}

// googleの認可コードからユーザを取得
pub async fn get_user_by_auth_code(
    context: &mut &common_struct::CommonContext,
    auth_code: String,
) -> Result<horse_model::AccountUserResponse> {
    // 認可コードからtokenを取得
    let auth_result = match (
        context.secrets.get("GOOGLE_AUTH_CLIENT_ID"),
        context.secrets.get("GOOGLE_AUTH_CLIENT_SECRET"),
        context.secrets.get("FRONT_DOMAIN"),
    ) {
        (Some(client_id), Some(client_secret), Some(redirect_url)) => {
            get_token_from_google_auth_code(auth_code, client_id, client_secret, redirect_url).await
        }
        (_, _, _) => {
            return Err(Error::new("Get google auth config Error")
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
    };

    // googleのtokenからgmailを取得しユーザを取得
    match (auth_result, context.secrets.get("JWT_SECRET")) {
        (Ok(google_token), Some(jwt_secret)) => {
            let gmail = google_token.email.unwrap();
            let find_result = account_users_repository::find_one_user_by_filter(
                context.mongo_db.clone(),
                doc! { "gmail": gmail.clone()},
            )
            .await;
            match find_result {
                Some(account_user) => {
                    // 認証用のトークンを生成
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
                None => {
                    return Err(Error::new("Not found user")
                        .extend_with(|_, e| e.set("type", ErrorType::AuthError)))
                }
            };
        }
        (Err(error), _) => return Err(error),
        (_, _) => {
            return Err(Error::new("Failed google auth")
                .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
        }
    };
}

// googleの認可コードからtokenを取得
async fn get_token_from_google_auth_code(
    auth_code: String,
    client_id: String,
    client_secret: String,
    redirect_url: String,
) -> Result<GoogleAccessTokenPayload> {
    // oauth用のクライアント
    let google_client_id = ClientId::new(client_id);
    let google_client_secret = ClientSecret::new(client_secret);
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string());
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string());
    let oauth_client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url?,
        Some(token_url?),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url)?);
    // 認証コードからトークン取得
    let token_result = oauth_client
        .exchange_code(AuthorizationCode::new(auth_code))
        .request_async(async_http_client)
        .await;

    let token = match token_result {
        Ok(t) => t,
        Err(error) => {
            return Err(Error::new(error.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::AuthError)))
        }
    };
    let access_token = token.access_token().secret();

    // アクセストークンからユーザ情報を取得
    let google_client = AsyncClient::new("");
    let payload_result = google_client.validate_access_token(access_token).await;
    let payload = match payload_result {
        Ok(t) => t,
        Err(error) => {
            return Err(Error::new(error.to_string())
                .extend_with(|_, e| e.set("type", ErrorType::AuthError)))
        }
    };

    Ok(payload)
}
