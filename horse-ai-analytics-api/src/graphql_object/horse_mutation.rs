use async_graphql::*;

use crate::service::auth::{account_user_service, google_auth_service};
use crate::service::race_info::{race_info_service, race_memo_category_service};
use crate::struct_const_def::common_struct;

use crate::graphql_object::horse_model;
use crate::graphql_object::horse_role::{Role, RoleGuard};

pub struct Mutation;

#[Object]
impl Mutation {
    // googleの認可コードから認証用のトークンを取得する処理
    async fn validate_google_auth_code(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] auth_code: String,
    ) -> Result<horse_model::ValidateGoogleAuthCodeResponse> {
        let context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        return google_auth_service::validate_auth_code(context, auth_code).await;
    }

    // googleの認可コードからユーザ認証する
    async fn login_google_auth_code(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] auth_code: String,
    ) -> Result<horse_model::AccountUserResponse> {
        let context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        return google_auth_service::get_user_by_auth_code(context, auth_code).await;
    }

    // google認証によるユーザの追加
    async fn add_account_user_from_google(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] auth_token: String,
        #[graphql(validator(min_length = 1))] user_setting_id: String,
        #[graphql(validator(min_length = 1))] name: String,
    ) -> Result<horse_model::AccountUserResponse> {
        let context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        return account_user_service::add_account_user_by_google_auth_token(
            context,
            auth_token,
            user_setting_id,
            name,
        )
        .await;
    }

    // ユーザ情報の編集
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn edit_account_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] user_setting_id: String,
        #[graphql(validator(min_length = 1))] name: String,
    ) -> Result<horse_model::AccountUserResponse> {
        let common_context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<common_struct::AuthContext>();
        return account_user_service::edit_account_user(
            common_context,
            auth_context.clone().account_id,
            user_setting_id,
            name,
        )
        .await;
    }

    // レース情報の追加
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn add_race_info(
        &self,
        ctx: &Context<'_>,
        input: horse_model::RaceInfoInputObject,
    ) -> Result<bool> {
        let common_context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<common_struct::AuthContext>();
        return race_info_service::add_race_info(
            common_context,
            auth_context.clone().account_id,
            input,
        )
        .await;
    }

    // レース情報の編集
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn edit_race_info(
        &self,
        ctx: &Context<'_>,
        input: horse_model::EditRaceInfoInputObject,
    ) -> Result<bool> {
        let common_context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<common_struct::AuthContext>();
        return race_info_service::edit_race_info(
            common_context,
            auth_context.clone().account_id,
            input,
        )
        .await;
    }

    // レース情報の削除
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn delete_race_info(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] race_info_id: String,
    ) -> Result<bool> {
        let common_context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<common_struct::AuthContext>();
        return race_info_service::delete_race_info(
            common_context,
            auth_context.clone().account_id,
            race_info_id,
        )
        .await;
    }

    // メモカテゴリーの追加
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn add_memo_category(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] name: String,
        #[graphql()] display_order: Option<i32>,
    ) -> Result<bool> {
        let common_context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<common_struct::AuthContext>();
        return race_memo_category_service::add_race_memo_category(
            common_context,
            auth_context.clone().account_id,
            name,
            display_order,
        )
        .await;
    }

    // メモカテゴリーの編集
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn edit_memo_category(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] id: String,
        #[graphql(validator(min_length = 1))] name: String,
        #[graphql()] display_order: Option<i32>,
    ) -> Result<bool> {
        let common_context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<common_struct::AuthContext>();
        return race_memo_category_service::edit_race_memo_category(
            common_context,
            auth_context.clone().account_id,
            id,
            name,
            display_order,
        )
        .await;
    }

    // メモカテゴリーの削除
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn delete_memo_category(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] id: String,
    ) -> Result<bool> {
        let common_context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<common_struct::AuthContext>();
        return race_memo_category_service::delete_race_memo_category(
            common_context,
            auth_context.clone().account_id,
            id,
        )
        .await;
    }
}
