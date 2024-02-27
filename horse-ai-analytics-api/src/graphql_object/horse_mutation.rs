use async_graphql::*;

use crate::graphql_object::horse_enum::ErrorType;
use crate::service::auth::google_auth_service;
use crate::struct_def::common_struct;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn validate_google_auth_code(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] auth_code: String,
    ) -> Result<String, Error> {
        let context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        let client_id_opt = context.secrets.get("GOOGLE_AUTH_CLIENT_ID");
        let client_secret_opt = context.secrets.get("GOOGLE_AUTH_CLIENT_SECRET");
        let redirect_url_opt = context.secrets.get("GOOGLE_AUTH_REDIRECT_URL");
        return match (client_id_opt, client_secret_opt, redirect_url_opt) {
            (Some(client_id), Some(client_secret), Some(redirect_url)) => {
                google_auth_service::validate_google_auth_code(
                    auth_code,
                    client_id,
                    client_secret,
                    redirect_url,
                )
                .await
            }
            (_, _, _) => Err(Error::new("Get google auth config Error")
                .extend_with(|_, e| e.set("type", ErrorType::SystemError))),
        };
    }
}
