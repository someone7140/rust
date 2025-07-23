use async_graphql::*;

use crate::model::common::context_info::{AuthContext, CommonContext};
use crate::model::graphql::graphql_guard::{Role, RoleGuard};
use crate::model::graphql::graphql_user_account;
use crate::service::auth::{google_auth_service, user_account_service};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // googleの認可コードからユーザ登録用のトークンを取得する処理
    async fn get_user_account_register_token_from_google_auth_code(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] auth_code: String,
    ) -> Result<graphql_user_account::RegisterTokenFromGoogleAuthCodeResponse> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        let google_access_token =
            match google_auth_service::get_google_access_token(context, auth_code).await {
                Ok(access_token) => access_token,
                Err(error) => return Err(error),
            };
        user_account_service::get_user_account_register_token(context, google_access_token).await
    }

    // ヘッダーから取得したユーザーIDでユーザ情報を取得
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn get_user_account_from_auth_header_user_account_id(
        &self,
        ctx: &Context<'_>,
    ) -> Result<graphql_user_account::UserAccountResponse> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<AuthContext>();

        user_account_service::get_user_account_by_id(context, auth_context.clone().user_account_id)
            .await
    }
}
