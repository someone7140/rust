use async_graphql::*;

use crate::model::common::context_info;
use crate::model::graphql::graphql_user_account;
use crate::service::auth::google_auth_service;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // googleの認可コードからユーザ登録用のトークンを取得する処理
    async fn get_user_account_register_token_from_google_auth_code(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] auth_code: String,
    ) -> Result<graphql_user_account::RegisterTokenFromGoogleAuthCodeResponse> {
        let context = &mut ctx.data_unchecked::<context_info::CommonContext>();
        return google_auth_service::get_account_register_token(context, auth_code).await;
    }
}
