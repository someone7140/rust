use async_graphql::*;

use crate::service::external_info::external_info_service;
use crate::{service::auth::account_user_service, struct_def::common_struct};

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
        return external_info_service::get_race_info_from_umanity_url(url).await;
    }
}
