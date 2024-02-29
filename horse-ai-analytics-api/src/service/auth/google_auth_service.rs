use async_graphql::*;
use google_oauth::{AsyncClient, GoogleAccessTokenPayload};
use mongodb::bson::doc;
use mongodb::Database;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl,
};

use crate::graphql_object::horse_enum::ErrorType;
use crate::repository::account_users_repository;
use crate::service::jwt_service;

// googleの認可コードからtokenを取得
pub async fn get_token_from_google_auth_code(
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

// gmailを元に認証用のtokenを生成
pub async fn make_for_register_auth_token(
    mongo_db: Database,
    jwt_secret: String,
    gmail: String,
) -> Result<String> {
    let find_result =
        account_users_repository::find_one_user_by_filter(mongo_db, doc! { "gmail": gmail.clone()})
            .await;
    let token = match find_result {
        Some(_) => {
            return Err(Error::new("Already registered user")
                .extend_with(|_, e| e.set("type", ErrorType::AlreadyExistsError)))
        }
        None => jwt_service::make_jwt(&jwt_secret, &gmail, 2),
    };

    Ok(token)
}
