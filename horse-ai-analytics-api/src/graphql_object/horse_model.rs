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

#[derive(SimpleObject)]
pub struct GetRaceInfoResponse {
    pub race_name: String,
    pub race_date_yyyy_mm_dd: String,
    pub prompt: String,
    pub odds: Option<OddsInfoResponse>,
}

#[derive(SimpleObject)]
pub struct OddsInfoResponse {
    pub odds_url: String,
    pub odds_list: Vec<OddsInfo>,
}

#[derive(SimpleObject)]
pub struct OddsInfo {
    pub horse_name: String,
    pub odds: String,
}
