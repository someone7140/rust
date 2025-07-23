use async_graphql::*;

use crate::model::common::context_info::CommonContext;
use crate::model::graphql::graphql_user_account;
use crate::service::auth::{google_auth_service, user_account_service};

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
}
