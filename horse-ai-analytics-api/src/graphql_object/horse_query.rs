use async_graphql::*;

use crate::{service::auth::account_user_service, struct_def::common_struct};

use crate::graphql_object::horse_model;
use crate::graphql_object::horse_role::{Role, RoleGuard};

pub struct Query;

#[Object]
impl Query {
    // ヘッダの認証トークンからからユーザを取得する
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
}
