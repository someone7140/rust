use async_graphql::*;

use crate::model::common::context_info;
use crate::model::graphql::graphql_user_account;
use crate::service::auth::user_account_service;

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
        let context = &mut ctx.data_unchecked::<context_info::CommonContext>();
        return user_account_service::add_user_account_by_gmail(
            context,
            register_token,
            user_setting_id,
            name,
        )
        .await;
    }
}
