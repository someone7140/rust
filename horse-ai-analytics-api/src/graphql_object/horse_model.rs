use async_graphql::*;

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

#[derive(InputObject)]
pub struct AddRaceInfoInputObject {
    #[graphql(validator(min_length = 1))]
    pub race_name: String,
    pub analytics_url: Option<String>,
    #[graphql(validator(min_length = 1))]
    pub race_date: String,
    pub prompt: Option<String>,
    pub memo_list: Vec<RaceMemoInputObject>,
}

#[derive(InputObject)]
pub struct RaceMemoInputObject {
    pub title: Option<String>,
    pub contents: Option<String>,
}

#[derive(InputObject)]
pub struct RaceInfoListFilterInputObject {
    pub start_race_date: Option<String>,
    pub end_race_date: Option<String>,
    pub keyword: Option<String>,
}

#[derive(SimpleObject)]
pub struct RaceInfoForList {
    pub id: String,
    pub race_name: String,
    pub race_date: String,
}

#[derive(SimpleObject)]
pub struct RaceInfoDetail {
    pub id: String,
    pub race_name: String,
    pub analytics_url: Option<String>,
    pub race_date: String,
    pub prompt: Option<String>,
    pub memo_list: Vec<RaceMemo>,
    pub odds: Option<OddsInfoResponse>,
}

#[derive(SimpleObject)]
pub struct RaceMemo {
    pub id: String,
    pub title: Option<String>,
    pub contents: Option<String>,
}
