use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

pub const ACCOUNT_USERS_COLLECTION: &str = "account_users";
pub const RACE_INFO_COLLECTION: &str = "race_info";
pub const RACE_MEMO_CATEGORY_COLLECTION: &str = "race_memo_categories";

#[derive(Serialize, Deserialize, Clone)]
pub struct AccountUsers {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub user_setting_id: String,
    pub gmail: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RaceInfo {
    #[serde(rename = "_id")]
    pub id: String,
    pub account_user_id: String,
    pub race_name: String,
    pub analytics_url: Option<String>,
    pub race_date: DateTime,
    pub prompt: Option<String>,
    pub memo_list: Vec<RaceInfoMemo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RaceInfoMemo {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: Option<String>,
    pub contents: Option<String>,
    pub create_date: DateTime,
    pub evaluation: Option<i32>,
    pub category_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RaceEvaluationAggregate {
    #[serde(rename = "_id")]
    pub key: RaceEvaluationAggregateKey,
    pub avg: f32,
    pub median: f32,
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RaceEvaluationAggregateKey {
    pub title: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RaceMemoCategory {
    #[serde(rename = "_id")]
    pub id: String,
    pub account_user_id: String,
    pub name: String,
    pub display_order: Option<i32>,
}
