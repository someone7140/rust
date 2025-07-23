use async_graphql::*;
use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{
    model::common::context_info::{AuthContext, CommonContext},
    service::auth::jwt_service::{self, JWT_USER_ACCOUNT_ID_KEY},
};

// JWT認証ミドルウェア
pub async fn jwt_auth_middleware(
    State(common_context): State<CommonContext>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Authorizationヘッダーからトークンを取得
    let user_account_id_opt = if let Some(auth_header) = headers.get("Authorization") {
        let auth_str = auth_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;
        if let (Some(token), Some(jwt_secret)) = (
            auth_str.strip_prefix("Bearer "),
            common_context.secrets.get("JWT_SECRET"),
        ) {
            match jwt_service::decode_jwt(token, &jwt_secret) {
                Ok(token_info) => token_info
                    .claims
                    .contents
                    .get(JWT_USER_ACCOUNT_ID_KEY)
                    .cloned(),
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            }
        } else {
            None
        }
    } else {
        None
    };

    // リクエストにユーザーIDを追加
    request
        .extensions_mut()
        .insert(user_account_id_opt.map(|id| AuthContext {
            user_account_id: id,
        }));

    Ok(next.run(request).await)
}
