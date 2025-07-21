use async_graphql::*;

#[derive(SimpleObject)]
pub struct RegisterTokenFromGoogleAuthCodeResponse {
    pub register_token: String,
}

#[derive(SimpleObject)]
pub struct UserAccountResponse {
    pub token: String,
    pub user_setting_id: String,
    pub name: String,
    pub image_url: Option<String>,
}
