use async_graphql::*;

use crate::service::external_info::external_info_main_service;
use crate::service::race_info::{race_info_service, race_memo_category_service};
use crate::{service::auth::account_user_service, struct_const_def::common_struct};

use crate::graphql_object::horse_model;
use crate::graphql_object::horse_role::{Role, RoleGuard};

pub struct Query;

#[Object]
impl Query {
    // ヘッダの認証トークンからユーザを取得する
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn get_user_from_auth_header(
        &self,
        ctx: &Context<'_>,
    ) -> Result<horse_model::AccountUserResponse> {
        let common_context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<common_struct::AuthContext>();
        return account_user_service::get_account_user_by_id(
            common_context,
            auth_context.clone().account_id,
        )
        .await;
    }

    // 指定したurlからレース情報やプロンプトを取得
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn get_race_info_from_url(
        &self,
        #[graphql(validator(min_length = 1))] url: String,
    ) -> Result<horse_model::GetRaceInfoResponse> {
        return external_info_main_service::get_race_info_from_umanity_url(url).await;
    }

    // 登録したレース情報の一覧
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn get_my_race_info_list(
        &self,
        ctx: &Context<'_>,
        filter: Option<horse_model::RaceInfoListFilterInputObject>,
    ) -> Result<Vec<horse_model::RaceInfoForList>> {
        let common_context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<common_struct::AuthContext>();
        return race_info_service::get_race_info_list_by_account(
            common_context,
            auth_context.clone().account_id,
            filter,
        )
        .await;
    }

    // レース情報の詳細
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn get_race_info_detail(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] race_info_id: String,
    ) -> Result<Option<horse_model::RaceInfoDetail>> {
        let common_context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<common_struct::AuthContext>();
        return race_info_service::get_race_info_detail(
            common_context,
            auth_context.clone().account_id,
            Some(race_info_id),
        )
        .await;
    }

    // 日付指定のレース情報の詳細
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn get_race_info_details_by_date(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] race_date: String,
    ) -> Result<Vec<horse_model::RaceInfoDetail>> {
        let common_context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<common_struct::AuthContext>();
        return race_info_service::get_race_info_details_by_date(
            common_context,
            auth_context.clone().account_id,
            race_date,
        )
        .await;
    }

    // レース回答の評価値集計
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn get_race_evaluation(
        &self,
        ctx: &Context<'_>,
        start_race_date_filter: Option<String>,
        end_race_date_filter: Option<String>,
    ) -> Result<Vec<horse_model::RaceEvaluationResult>> {
        let common_context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<common_struct::AuthContext>();
        return race_info_service::get_race_evaluation(
            common_context,
            auth_context.clone().account_id,
            start_race_date_filter,
            end_race_date_filter,
        )
        .await;
    }

    // メモカテゴリーの一覧
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    async fn get_race_memo_category_list(
        &self,
        ctx: &Context<'_>,
        id_filter: Option<String>,
    ) -> Result<Vec<horse_model::RaceMemoCategory>> {
        let common_context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        let auth_context = &mut ctx.data_unchecked::<common_struct::AuthContext>();
        return race_memo_category_service::get_race_memo_category_list_by_account(
            common_context,
            auth_context.clone().account_id,
            id_filter,
        )
        .await;
    }
}
