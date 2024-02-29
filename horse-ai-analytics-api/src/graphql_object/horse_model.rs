use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct ValidateGoogleAuthCodeResponse {
    pub auth_token: String,
}
