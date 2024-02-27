use async_graphql::*;
use google_oauth::AsyncClient;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, PkceCodeChallenge, RedirectUrl,
    TokenResponse, TokenUrl,
};

use crate::graphql_object::horse_enum::ErrorType;

pub async fn validate_google_auth_code(
    auth_code: String,
    client_id: String,
    client_secret: String,
    redirect_url: String,
) -> Result<String> {
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
    let payload = google_client
        .validate_access_token(access_token)
        .await
        .unwrap();

    Ok(payload.name.unwrap())
}
