use async_graphql::*;

use crate::graphql_object::horse_enum::ErrorType;
use crate::service::auth::google_auth_service;
use crate::struct_def::common_struct;

use crate::graphql_object::horse_model;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn validate_google_auth_code(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(min_length = 1))] auth_code: String,
    ) -> Result<horse_model::ValidateGoogleAuthCodeResponse> {
        let context = &mut ctx.data_unchecked::<common_struct::CommonContext>();
        // 認証コードからtokenを取得
        let auth_result = match (
            context.secrets.get("GOOGLE_AUTH_CLIENT_ID"),
            context.secrets.get("GOOGLE_AUTH_CLIENT_SECRET"),
            context.secrets.get("GOOGLE_AUTH_REDIRECT_URL"),
        ) {
            (Some(client_id), Some(client_secret), Some(redirect_url)) => {
                google_auth_service::get_token_from_google_auth_code(
                    auth_code,
                    client_id,
                    client_secret,
                    redirect_url,
                )
                .await
            }
            (_, _, _) => {
                return Err(Error::new("Get google auth config Error")
                    .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
            }
        };
        // googleのtokenからgmailを取得し認証用トークンを生成
        let auth_token_result = match (auth_result, context.secrets.get("JWT_SECRET")) {
            (Ok(google_token), Some(jwt_secret)) => {
                google_auth_service::make_for_register_auth_token(
                    context.mongo_db.clone(),
                    jwt_secret,
                    google_token.email.unwrap(),
                )
                .await
            }
            (Err(error), _) => return Err(error),
            (_, _) => {
                return Err(Error::new("Failed google auth")
                    .extend_with(|_, e| e.set("type", ErrorType::SystemError)))
            }
        };

        match auth_token_result {
            Ok(auth_token) => {
                return Ok(horse_model::ValidateGoogleAuthCodeResponse { auth_token })
            }
            Err(error) => return Err(error),
        };
    }
}
