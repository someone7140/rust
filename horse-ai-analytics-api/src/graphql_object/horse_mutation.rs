use async_graphql::*;

use crate::service::auth::{account_user_service, google_auth_service};
use crate::struct_const_def::common_struct;

use crate::graphql_object::horse_model;

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
}
