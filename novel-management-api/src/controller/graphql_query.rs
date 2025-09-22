use async_graphql::*;

use crate::model::common::context_info::{AuthContext, CommonContext};
use crate::model::graphql::graphql_guard::{Role, RoleGuard};
use crate::model::graphql::{graphql_novel, graphql_novel_setting, graphql_user_account};
use crate::service::auth::{google_auth_service, user_account_service};
use crate::service::novel::{novel_service, novel_setting_service};

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
    async fn get_user_account_from_auth_header(
        &self,
        ctx: &Context<'_>,
    ) -> Result<graphql_user_account::UserAccountResponse> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<AuthContext>();

        user_account_service::get_user_account_by_id(context, auth_context.clone().user_account_id)
            .await
    }

    // 小説の一覧を取得
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn get_my_novels(&self, ctx: &Context<'_>) -> Result<Vec<graphql_novel::NovelResponse>> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<AuthContext>();

        novel_service::get_my_novels(context, auth_context.clone().user_account_id).await
    }

    // 小説の設定一覧を取得
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn get_my_novel_settings(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] novel_id: String,
    ) -> Result<Vec<graphql_novel_setting::NovelSettingResponse>> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<AuthContext>();

        novel_setting_service::get_novel_settings(
            context,
            auth_context.clone().user_account_id,
            novel_id,
        )
        .await
    }

    // 親設定をキーにした設定一覧を取得
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn get_novel_settings_by_parent_setting_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] parent_setting_id: String,
    ) -> Result<Vec<graphql_novel_setting::NovelSettingResponse>> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<AuthContext>();

        novel_setting_service::get_novel_settings_by_parent_setting_id(
            context,
            auth_context.clone().user_account_id,
            parent_setting_id,
        )
        .await
    }
}
