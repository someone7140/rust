use async_graphql::*;
use google_oauth::{AsyncClient, GoogleAccessTokenPayload};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenResponse, TokenUrl,
};

use crate::model::common::context_info;
use crate::model::graphql::graphql_error::AppError;

// googleのアクセストークンを取得
pub async fn get_google_access_token(
    context: &mut &context_info::CommonContext,
    auth_code: String,
) -> Result<GoogleAccessTokenPayload> {
    // 認可コードからtokenを取得
    match (
        context.secrets.get("GOOGLE_AUTH_CLIENT_ID"),
        context.secrets.get("GOOGLE_AUTH_CLIENT_SECRET"),
        context.secrets.get("FRONT_DOMAIN"),
    ) {
        (Some(client_id), Some(client_secret), Some(redirect_url)) => {
            get_token_from_google_auth_code(auth_code, client_id, client_secret, redirect_url).await
        }
        (_, _, _) => {
            return Err(AppError::SystemError("Get google auth config Error".to_string()).extend())
        }
    }
}

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
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())?;
    let oauth_client = BasicClient::new(google_client_id)
        .set_client_secret(google_client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(RedirectUrl::new(redirect_url)?);
    let http_client = reqwest::Client::new();

    // 認証コードからトークン取得
    let token_result = oauth_client
        .exchange_code(AuthorizationCode::new(auth_code))
        .request_async(&http_client)
        .await;

    let token = match token_result {
        Ok(t) => t,
        Err(error) => return Err(AppError::AuthorizationError(error.to_string()).extend()),
    };
    let access_token = token.access_token().secret();

    // アクセストークンからユーザ情報を取得
    let google_client = AsyncClient::new("");
    let payload_result = google_client.validate_access_token(access_token).await;
    let payload = match payload_result {
        Ok(t) => t,
        Err(error) => return Err(AppError::AuthorizationError(error.to_string()).extend()),
    };

    Ok(payload)
}
