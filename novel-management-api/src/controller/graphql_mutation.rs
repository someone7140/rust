use async_graphql::*;

use crate::model::common::context_info::{AuthContext, CommonContext};
use crate::model::graphql::graphql_guard::{Role, RoleGuard};
use crate::model::graphql::graphql_novel_setting;
use crate::model::graphql::graphql_user_account;
use crate::service::auth::{google_auth_service, user_account_service};
use crate::service::novel::{novel_service, novel_setting_service};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // google認証によるユーザの追加
    async fn add_user_account_by_google_auth(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] register_token: String,
        #[graphql(validator(min_length = 1))] user_setting_id: String,
        #[graphql(validator(min_length = 1))] name: String,
    ) -> Result<graphql_user_account::UserAccountResponse> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        user_account_service::add_user_account_by_gmail(
            context,
            register_token,
            user_setting_id,
            name,
        )
        .await
    }

    // google認証によるログイン
    async fn login_by_google_auth(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] auth_code: String,
    ) -> Result<graphql_user_account::UserAccountResponse> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        let google_access_token =
            match google_auth_service::get_google_access_token(context, auth_code).await {
                Ok(access_token) => access_token,
                Err(error) => return Err(error),
            };

        user_account_service::get_user_account_by_google_token(context, google_access_token).await
    }

    // ユーザー情報の編集
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn edit_user_account(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] user_setting_id: String,
        #[graphql(validator(min_length = 1))] name: String,
    ) -> Result<graphql_user_account::UserAccountResponse> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<AuthContext>();

        user_account_service::edit_user_account(
            context,
            auth_context.clone().user_account_id,
            user_setting_id,
            name,
        )
        .await
    }

    // 小説の作成
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn add_novel(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] title: String,
        description: Option<String>,
    ) -> Result<bool> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<AuthContext>();

        novel_service::add_novel(
            context,
            auth_context.clone().user_account_id,
            title,
            description,
        )
        .await
    }

    // 小説の編集
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn edit_novel(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] id: String,
        #[graphql(validator(min_length = 1))] title: String,
        description: Option<String>,
    ) -> Result<bool> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<AuthContext>();

        novel_service::edit_novel(
            context,
            auth_context.clone().user_account_id,
            id,
            title,
            description,
        )
        .await
    }

    // 小説の削除
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn delete_novel(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] id: String,
    ) -> Result<bool> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<AuthContext>();

        novel_service::delete_novel(context, auth_context.clone().user_account_id, id).await
    }

    // 小説設定の登録
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn register_novel_settings(
        &self,
        ctx: &Context<'_>,
        inputs: Vec<graphql_novel_setting::NovelSettingRegisterInput>,
    ) -> Result<bool> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<AuthContext>();

        novel_setting_service::register_novel_settings(
            context,
            auth_context.clone().user_account_id,
            inputs,
        )
        .await
    }

    // 小説設定の削除
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn delete_novel_settings_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] setting_id: String,
    ) -> Result<bool> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<AuthContext>();

        novel_setting_service::delete_novel_settings_by_id(
            context,
            auth_context.clone().user_account_id,
            setting_id,
        )
        .await
    }

    // 小説設定の削除（複数指定）
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn delete_novel_settings_by_ids(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_items = 1))] setting_ids: Vec<String>,
    ) -> Result<bool> {
        let context = &mut ctx.data_unchecked::<CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<AuthContext>();

        novel_setting_service::delete_novel_settings_by_ids(
            context,
            auth_context.clone().user_account_id,
            setting_ids,
        )
        .await
    }
}
