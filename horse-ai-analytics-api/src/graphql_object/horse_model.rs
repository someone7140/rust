use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct AccountUserResponse {
    pub auth_token: Option<String>,
    pub user_setting_id: String,
    pub name: String,
}

#[derive(SimpleObject)]
pub struct ValidateGoogleAuthCodeResponse {
    pub auth_token: String,
}
